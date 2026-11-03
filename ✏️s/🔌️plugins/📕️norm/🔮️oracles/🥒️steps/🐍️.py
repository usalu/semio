"""🥒️ The shared, domain-neutral step layer of every norm `🥒️.feature`, Python side.

Schema-first: the step grammar lives in `🧬️schema/🔣️.json` and the Rust twin `🦀️.rs` reads the same file. A
feature names one family by its step wording; this module binds the holes, runs the independent oracle through
`run_oracle` and builds the projection both implementations emit, so no artifact adapter re-implements step
parsing, tolerances or oracle plumbing. Artifact adapters contribute only a `Binding`.

@see https://cucumber.io/docs/gherkin/reference/
"""

from __future__ import annotations

import importlib.util
import json
import math
import os
import subprocess
import sys
from dataclasses import dataclass
from typing import Any, Callable, Dict, List, Optional, Tuple

HERE = os.path.dirname(os.path.abspath(__file__))
GRAMMAR_PATH = os.path.join(HERE, "🧬️schema", "🔣️.json")


def grammar() -> Dict[str, Any]:
    """📖️ The committed step grammar."""
    with open(GRAMMAR_PATH, encoding="utf8") as handle:
        return json.load(handle)["families"]


def match_template(template: str, text: str) -> Optional[Dict[str, str]]:
    """🔎️ Binds the `{name}` holes of `template` to `text` by finding its literal segments in order; `None` when the wording differs."""
    segments: List[Tuple[str, str]] = []
    rest = template
    while rest:
        start = rest.find("{")
        if start < 0:
            segments.append(("lit", rest))
            break
        if start:
            segments.append(("lit", rest[:start]))
        end = rest.index("}", start)
        segments.append(("hole", rest[start + 1 : end]))
        rest = rest[end + 1 :]
    bound: Dict[str, str] = {}
    at = 0
    for index, (kind, value) in enumerate(segments):
        if kind == "lit":
            if index == 0 and not text.startswith(value):
                return None
            found = text.find(value, at) if index else 0
            if found < 0 or (index == len(segments) - 1 and not text.endswith(value)):
                return None
            at = found + len(value)
            continue
        following = segments[index + 1][1] if index + 1 < len(segments) else None
        if following is None:
            bound[value], at = text[at:], len(text)
        else:
            stop = text.find(following, at)
            if stop < 0:
                return None
            bound[value], at = text[at:stop], stop
    return bound if at == len(text) else None


def parse_steps(steps: List[Dict[str, Any]]) -> Tuple[str, Dict[str, str]]:
    """🧩️ The family a scenario's steps spell and every bound hole; an unknown wording is an error, never a fallback."""
    families = grammar()
    texts = [step["text"] for step in steps]
    for name, family in families.items():
        templates = [entry["template"] for entry in family["steps"]]
        if len(templates) != len(texts):
            continue
        fields: Dict[str, str] = {}
        for template, text in zip(templates, texts):
            hit = match_template(template, text)
            if hit is None:
                break
            fields.update(hit)
        else:
            return name, fields
    raise AssertionError("steps %r spell no norm step family; the wording must be one of 🧬️schema/🔣️.json" % texts)


def parse_inputs(text: str) -> Tuple[Dict[str, float], Dict[str, str]]:
    """🔢️ `a=1.5;b:text` into numeric and text inputs (SI numbers `name=value`, text `name:value`, separated by semicolons)."""
    numbers: Dict[str, float] = {}
    texts: Dict[str, str] = {}
    for part in [piece for piece in text.split(";") if piece.strip()]:
        if "=" in part:
            name, value = part.split("=", 1)
            numbers[name.strip()] = float(value)
        else:
            name, value = part.split(":", 1)
            texts[name.strip()] = value.strip()
    return numbers, texts


def within(actual: float, expected: float, tolerance: float) -> bool:
    """⚖️ Relative closeness with an absolute floor of the tolerance for an expected zero, the rule every projection profile shares."""
    return abs(actual - expected) <= tolerance * max(abs(expected), abs(actual), 1e-12) or abs(actual - expected) <= 1e-12


def require_within(label: str, actual: float, expected: float, tolerance: float) -> None:
    """🛑️ Fails the scenario with both numbers when `actual` leaves the tolerance."""
    if not within(actual, expected, tolerance):
        raise AssertionError("%s: got %r, expected %r within relative %r" % (label, actual, expected, tolerance))


def interpreter() -> str:
    """🐍️ The repo-resolved interpreter the host already runs on."""
    return sys.executable


def run_oracle(script: str, args: List[str], request: Optional[Any] = None) -> Any:
    """🔮️ Runs an independent Python oracle script isolated (`-I`), feeds `request` as JSON on stdin and returns its parsed JSON stdout."""
    done = subprocess.run([interpreter(), "-I", script, *args], input=None if request is None else json.dumps(request).encode("utf8"), capture_output=True, check=False)
    if done.returncode != 0:
        raise AssertionError("oracle %s failed: %s" % (script, done.stderr.decode("utf8", "replace")))
    return json.loads(done.stdout.decode("utf8"))


def run_oracle_text(script: str, args: List[str], text: str) -> str:
    """🔮️ Runs an independent Python oracle isolated (`-I`) with a line protocol: `text` on stdin, its stdout returned verbatim."""
    done = subprocess.run([interpreter(), "-I", script, *args], input=text.encode("utf8"), capture_output=True, check=False)
    if done.returncode != 0:
        raise AssertionError("oracle %s failed: %s" % (script, done.stderr.decode("utf8", "replace")))
    return done.stdout.decode("utf8")


def load_module(alias: str, path: str) -> Any:
    """📦️ Loads a module from a path whose file name is not an identifier."""
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


def number_map(values: Dict[str, Any]) -> Dict[str, float]:
    """🔢️ The numeric members of an oracle answer in key order, so a projection never carries prose or booleans."""
    return {key: float(value) for key, value in sorted(values.items()) if isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(float(value))}


@dataclass(frozen=True)
class Binding:
    """🔌️ What one artifact contributes: how its fixture is read and how its oracle answers."""

    fixture_cases: Callable[[str], List[Dict[str, Any]]] = lambda path: []
    oracle_case: Callable[[Dict[str, Any]], Dict[str, float]] = lambda case: {}
    oracle_example: Callable[[str, str, Dict[str, float], Dict[str, str]], Dict[str, float]] = lambda entry, annex, numbers, texts: {}
    oracle_vector: Callable[[str, str], float] = lambda vector, quantity: math.nan
    oracle_utilisation: Callable[[str, str, str, str], float] = lambda subject, annex, check, quantity: math.nan
    oracle_check: Callable[[str, str], Dict[str, Any]] = lambda fixture, check: {}
    oracle_scene: Callable[[str, str], Dict[str, Any]] = lambda fixture, entity: {}
    oracle_round_trip: Callable[[str, str], bool] = lambda before, mutation: False
    oracle_comparison: Callable[[str], Dict[str, float]] = lambda path: {}


def pass_or_fail(utilisation: float) -> str:
    """🚦️ The status of a utilisation the way the compliance report assigns it away from the near-limit band: at most one passes, above one fails."""
    return "pass" if utilisation <= 1.0 else "fail"


def check_outcome(rows: Dict[str, Tuple[str, float]], check: str) -> Dict[str, Any]:
    """🎯️ The `{status, utilisation}` of `check` in `rows` (id to status and utilisation): the exact id, else the worst row of every id below it (`<check>.<suffix>`), so a per-load-case family is judged by its maximum."""
    if check in rows:
        status, utilisation = rows[check]
        return {"status": status, "utilisation": utilisation}
    below = [row for key, row in rows.items() if key.startswith(check + ".")]
    if not below:
        raise AssertionError("the oracle reports no check %r" % check)
    utilisation = max(utilisation for _, utilisation in below)
    return {"status": "fail" if any(status == "fail" for status, _ in below) else "warning" if any(status == "warning" for status, _ in below) else "pass", "utilisation": utilisation}


def vocabulary_round_trip(kinds: List[str]) -> Callable[[str, str], bool]:
    """↩️ The `oracle_round_trip` of a subset whose closed `kinds` the shared mutation vocabulary engine (`../🏃️execution`) applies and inverts independently of the Rust producer."""
    import re
    from importlib import import_module

    def round_trip(before_path: str, mutation_path: str) -> bool:
        engine = import_module("🐍️")
        base = json.load(open(before_path, encoding="utf8"))
        tag, arguments = engine.unwrap(json.load(open(mutation_path, encoding="utf8")))
        kind = re.sub(r"(?<!^)(?=[A-Z])", "-", tag).lower()
        if kind not in kinds:
            raise AssertionError("mutation %s is not a kind of this subset" % kind)
        mutated = engine.apply_mutation(base, kind, arguments)
        steps = engine.inverse_mutation(kinds, base, kind, arguments)
        view = engine.derived_view(base)
        restored = mutated
        for step, payload in steps:
            restored = engine.apply_mutation(restored, step, payload, view)
        return restored == base and mutated != base and bool(steps)

    return round_trip


def projection(family: str, **fields: Any) -> Dict[str, Any]:
    """🧾️ The canonical projection of a family; the keys are exactly the grammar's, in grammar order."""
    shape = grammar()[family]["projection"]
    missing = [key for key in shape if key not in fields]
    if missing or set(fields) - set(shape):
        raise AssertionError("projection of %s needs exactly %s, got %s" % (family, list(shape), list(fields)))
    return {key: fields[key] for key in shape}


def oracle_handler(binding: Binding) -> Callable[[Any], Any]:
    """🔮️ The oracle-role handler for any norm family: one function dispatching on the family the scenario's steps spell."""
    from semio_repo_test import Outcome

    def handle(ctx: Any) -> Any:
        family, fields = parse_steps(ctx.scenario["steps"])
        if family == "catalogue-fixture-case":
            path = ctx.input(fields["fixture"])
            index = int(fields["index"])
            case = binding.fixture_cases(path)[index]
            if case["entry"] != fields["entry"]:
                raise AssertionError("fixture index %d holds %s, the feature says %s" % (index, case["entry"], fields["entry"]))
            body = projection(family, entry=fields["entry"], index=index, values=number_map(binding.oracle_case(case)))
        elif family == "catalogue-example":
            numbers, texts = parse_inputs(fields["inputs"])
            answer = binding.oracle_example(fields["entry"], fields["annex"], numbers, texts)
            value = float(answer[fields["key"]] if fields["key"] in answer else answer[fields["key"].replace("-", "_")])
            require_within("%s %s" % (fields["entry"], fields["key"]), value, float(fields["expected"]), float(fields["tolerance"]))
            body = projection(family, entry=fields["entry"], annex=fields["annex"], key=fields["key"], value=value)
        elif family == "clause-vector":
            value = float(binding.oracle_vector(fields["vector"], fields["quantity"]))
            body = projection(family, vector=fields["vector"], quantity=fields["quantity"], value=value)
        elif family == "published-example":
            value = float(binding.oracle_vector(fields["example"], fields["quantity"]))
            require_within("%s %s" % (fields["example"], fields["quantity"]), value, float(fields["expected"]), float(fields["tolerance"]))
            body = projection(family, example=fields["example"], quantity=fields["quantity"], value=value)
        elif family == "oracle-check":
            value = float(binding.oracle_utilisation(fields["subject"], fields["annex"], fields["check"], fields["quantity"]))
            body = projection(family, check=fields["check"], quantity=fields["quantity"], utilisation=value)
        elif family == "check-outcome":
            answer = binding.oracle_check(ctx.input(fields["fixture"]), fields["check"])
            require_within("%s utilisation" % fields["check"], float(answer["utilisation"]), float(fields["utilisation"]), float(fields["tolerance"]))
            if answer["status"] != fields["status"]:
                raise AssertionError("%s status %r, expected %r" % (fields["check"], answer["status"], fields["status"]))
            body = projection(family, check=fields["check"], status=answer["status"], utilisation=float(answer["utilisation"]))
        elif family == "scene-law":
            answer = binding.oracle_scene(ctx.input(fields["fixture"]), fields["entity"])
            body = projection(family, entity=fields["entity"], kind=answer["kind"], handles=int(answer["handles"]), diagrams=int(answer["diagrams"]))
        elif family == "mutation-round-trip":
            restored = bool(binding.oracle_round_trip(ctx.input(fields["before"]), ctx.input(fields["mutation"])))
            if not restored:
                raise AssertionError("applying %s and then its computed inverse did not restore %s" % (fields["mutation"], fields["before"]))
            body = projection(family, mutation=fields["mutation"], restored=restored)
        else:
            body = projection(family, fixture=fields["fixture"], values=number_map(binding.oracle_comparison(ctx.input(fields["fixture"]))))
        return Outcome(body, raw=json.dumps(body, sort_keys=True).encode("utf8"))

    return handle


def norm_adapter(binding: Binding) -> Any:
    """🧭️ The oracle-role adapter of a norm case; the subject role is the Rust adapter, so the reference is never its own subject."""
    from semio_repo_test import Adapter

    handler = oracle_handler(binding)
    adapter = Adapter("python")
    for family in grammar():
        adapter.oracle(family, handler)
    return adapter
