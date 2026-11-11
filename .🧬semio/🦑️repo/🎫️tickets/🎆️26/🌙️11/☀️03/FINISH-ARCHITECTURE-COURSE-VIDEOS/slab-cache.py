"""🧱 Ticket prototype for exact, cross-process BIM slab geometry caching."""
import contextlib
import errno
import hashlib
import inspect
import sys
import json
import math
import os
import time
import uuid
from fractions import Fraction
from pathlib import Path

if os.name == "nt":
    import msvcrt
else:
    import fcntl

REVISION = "semio-exact-slab-cache/v1"


def encode(value):
    """🧬 Encode exact domain values without confusing user data with tags."""
    if value is None or isinstance(value, (bool, str)):
        return ["scalar", value]
    if type(value) is int:
        return ["integer", str(value)]
    if isinstance(value, Fraction):
        return ["fraction", str(value.numerator), str(value.denominator)]
    if isinstance(value, float):
        if not math.isfinite(value):
            raise ValueError("Non-finite values cannot be cached")
        return ["float", value.hex()]
    if isinstance(value, (list, tuple)):
        return ["tuple" if isinstance(value, tuple) else "list", [encode(item) for item in value]]
    if isinstance(value, dict):
        items = [[encode(key), encode(item)] for key, item in value.items()]
        items.sort(key=lambda item: canonical(item[0]))
        return ["dict", items]
    raise TypeError(f"Unsupported cache value: {type(value).__name__}")


def decode(node):
    """📦 Decode only valid tagged values; malformed data is never reused."""
    if not isinstance(node, list) or not node or not isinstance(node[0], str):
        raise ValueError("Invalid cache node")
    tag = node[0]
    if tag == "scalar" and len(node) == 2 and (node[1] is None or type(node[1]) in (bool, str)):
        return node[1]
    if tag == "integer" and len(node) == 2 and isinstance(node[1], str):
        value = int(node[1])
        if str(value) != node[1]:
            raise ValueError("Noncanonical integer")
        return value
    if tag == "fraction" and len(node) == 3 and all(isinstance(x, str) for x in node[1:]):
        value = Fraction(int(node[1]), int(node[2]))
        if str(value.numerator) != node[1] or str(value.denominator) != node[2]:
            raise ValueError("Noncanonical fraction")
        return value
    if tag == "float" and len(node) == 2 and isinstance(node[1], str):
        value = float.fromhex(node[1])
        if not math.isfinite(value) or value.hex() != node[1]:
            raise ValueError("Invalid float")
        return value
    if tag in ("tuple", "list") and len(node) == 2 and isinstance(node[1], list):
        items = [decode(item) for item in node[1]]
        return tuple(items) if tag == "tuple" else items
    if tag == "dict" and len(node) == 2 and isinstance(node[1], list):
        result = {}
        for pair in node[1]:
            if not isinstance(pair, list) or len(pair) != 2:
                raise ValueError("Invalid dictionary entry")
            key = decode(pair[0])
            if key in result:
                raise ValueError("Duplicate dictionary key")
            result[key] = decode(pair[1])
        return result
    raise ValueError("Invalid cache tag")


def canonical(value):
    """🧾 Stable UTF-8 JSON for cache keys and integrity checks."""
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def digest(value):
    """🔏 SHA-256 of a canonical JSON value."""
    return hashlib.sha256(canonical(value).encode("utf-8")).hexdigest()


def identity(model, slab, root: Path, producers, producer_slots=()):
    """🏷️ Key the full model, slab and declared producer files by portable names and contents."""
    root = Path(root).resolve()
    sources = {}
    for source in producers:
        path = Path(source).resolve()
        relative = path.relative_to(root).as_posix()
        sources[relative] = hashlib.sha256(path.read_bytes()).hexdigest()
    if not sources:
        raise ValueError("Producer sources must be declared")
    return digest({"revision": REVISION, "model": encode(model), "slab": encode(slab), "sources": sources, "producerSlots": encode(tuple(producer_slots))})


def register_producers(bindings, expected_slots, root, producers, loaded_sources):
    """📌 Capture canonical loaded producer identities at module initialization."""
    bindings = tuple(bindings)
    expected_slots = tuple(expected_slots)
    source_files = {Path(path).resolve() for path in producers}
    snapshot = []
    for binding in bindings:
        function = inspect.unwrap(binding)
        slot = (getattr(function, "__module__", ""), getattr(function, "__qualname__", ""))
        if not inspect.isfunction(function) or function.__closure__ or "<locals>" in slot[1]:
            return None
        try:
            owner = sys.modules[slot[0]]
            for name in slot[1].split("."):
                owner = getattr(owner, name)
            if inspect.unwrap(owner) is not function or Path(inspect.getsourcefile(function)).resolve() not in source_files:
                return None
        except (AttributeError, KeyError, TypeError):
            return None
        snapshot.append((binding, function, function.__code__, slot))
    if tuple(item[3] for item in snapshot) != expected_slots:
        return None
    root = Path(root).resolve()
    fingerprints = {path: hashlib.sha256(path.read_bytes()).hexdigest() for path in source_files}
    if any(not path.is_relative_to(root) for path in fingerprints):
        return None
    if {str(path): value for path, value in fingerprints.items()} != loaded_sources:
        return None
    return (tuple(snapshot), fingerprints)


def producers_current(registry, bindings):
    """🛡️ Decline persistence if a loaded producer or its registered source changes."""
    if registry is None:
        return False
    snapshot, fingerprints = registry
    bindings = tuple(bindings)
    if len(bindings) != len(snapshot):
        return False
    try:
        for binding, (old, function, code, slot) in zip(bindings, snapshot, strict=True):
            current = inspect.unwrap(binding)
            if binding is not old or current is not function or current.__code__ is not code or current.__closure__:
                return False
            owner = sys.modules[slot[0]]
            for name in slot[1].split("."):
                owner = getattr(owner, name)
            if inspect.unwrap(owner) is not function:
                return False
        return all(path.is_file() and hashlib.sha256(path.read_bytes()).hexdigest() == expected for path, expected in fingerprints.items())
    except (AttributeError, KeyError, OSError, TypeError):
        return False


@contextlib.contextmanager
def lock(path: Path, cancelled, progress):
    """🔒 Use a process-death-safe operating-system lock with cancellable waiting."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a+b") as handle:
        if path.stat().st_size == 0:
            handle.write(b"0")
            handle.flush()
        reported = False
        while True:
            if cancelled():
                raise InterruptedError("Slab cache wait cancelled")
            try:
                handle.seek(0)
                if os.name == "nt":
                    msvcrt.locking(handle.fileno(), msvcrt.LK_NBLCK, 1)
                else:
                    fcntl.flock(handle.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
                break
            except OSError as error:
                if error.errno not in {errno.EACCES, errno.EAGAIN, errno.EDEADLK} and getattr(error, "winerror", None) not in {33, 36}:
                    raise
                if not reported:
                    progress("waiting")
                    reported = True
                time.sleep(0.05)
        try:
            yield
        finally:
            handle.seek(0)
            if os.name == "nt":
                msvcrt.locking(handle.fileno(), msvcrt.LK_UNLCK, 1)
            else:
                fcntl.flock(handle.fileno(), fcntl.LOCK_UN)


def get_or_compute(model, slab, root, producers, folder, build, *, eligibility, producer_slots, cancelled=lambda: False, progress=lambda phase: None):
    """♻️ Reuse an exact result or publish a complete result only under its unchanged key."""
    producers = tuple(producers)
    if not eligibility():
        if cancelled():
            raise InterruptedError("Slab cache cancelled")
        progress("ineligible")
        return build(cancelled, progress)
    key = identity(model, slab, root, producers, producer_slots)
    folder = Path(folder)
    path = folder / (key + ".json")
    with lock(folder / (key + ".lock"), cancelled, progress):
        if cancelled():
            raise InterruptedError("Slab cache cancelled")
        if not eligibility() or identity(model, slab, root, producers, producer_slots) != key:
            raise RuntimeError("Slab cache inputs changed while waiting")
        if path.is_file():
            try:
                data = json.loads(path.read_text(encoding="utf-8"))
                if not isinstance(data, dict) or set(data) != {"revision", "key", "value", "digest"} or data["revision"] != REVISION or data["key"] != key or digest(data["value"]) != data["digest"]:
                    raise ValueError("Invalid slab cache record")
                result = decode(data["value"])
            except (ValueError, TypeError, KeyError, ZeroDivisionError, OverflowError):
                path.unlink(missing_ok=True)
                progress("corrupt")
            else:
                if cancelled():
                    raise InterruptedError("Slab cache cancelled")
                if not eligibility() or identity(model, slab, root, producers, producer_slots) != key:
                    raise RuntimeError("Slab cache inputs changed while decoding")
                progress("hit")
                return result
        progress("compute")
        result = build(cancelled, progress)
        if cancelled():
            raise InterruptedError("Slab cache cancelled")
        if not eligibility() or identity(model, slab, root, producers, producer_slots) != key:
            raise RuntimeError("Slab cache inputs changed while computing")
        value = encode(result)
        record = {"revision": REVISION, "key": key, "value": value, "digest": digest(value)}
        temporary = folder / ("." + uuid.uuid4().hex + ".tmp")
        try:
            temporary.write_text(canonical(record), encoding="utf-8")
            if cancelled():
                raise InterruptedError("Slab cache cancelled")
            if not eligibility() or identity(model, slab, root, producers, producer_slots) != key:
                raise RuntimeError("Slab cache inputs changed while encoding")
            os.replace(temporary, path)
        finally:
            temporary.unlink(missing_ok=True)
        progress("published")
        return result
