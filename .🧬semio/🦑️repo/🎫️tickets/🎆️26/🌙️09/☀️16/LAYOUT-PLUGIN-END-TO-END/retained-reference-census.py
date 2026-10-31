"""🔎️ Bounded read-only retained generation reference census; no payload walk, readiness or effect authority."""
import os, sys, json, stat, re, hashlib, time
from pathlib import Path

workspace = Path.cwd().resolve()
ticket = Path(__file__).resolve().parent
root = workspace / ".cache/b/d/m"
container = root / "j"
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
started = time.monotonic()
limits = {"generations":4096,"receiptBytes":4194304,"aggregateBytes":67108864,"sources":4096,"seconds":30}
errors, rows, aliases = [], [], []
aggregate = 0

def stamp(value):
    return [str(getattr(value,key)) for key in ("st_dev","st_ino","st_mode","st_nlink","st_size","st_mtime_ns","st_ctime_ns")]

def check():
    if time.monotonic() - started > limits["seconds"]:
        raise ValueError("bounded census elapsed limit")

def pairs(values):
    result = {}
    for key, value in values:
        if key in result:
            raise ValueError("duplicate JSON field")
        result[key] = value
    return result

def read_receipt(path):
    global aggregate
    check()
    before = os.lstat(path)
    if not stat.S_ISREG(before.st_mode) or before.st_nlink != 1 or not 0 < before.st_size <= limits["receiptBytes"]:
        raise ValueError("bounded ordinary single-owner receipt")
    aggregate += before.st_size
    if aggregate > limits["aggregateBytes"]:
        raise ValueError("aggregate receipt capacity")
    descriptor = os.open(path, os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0))
    try:
        if stamp(os.fstat(descriptor)) != stamp(before):
            raise ValueError("receipt descriptor changed")
        chunks, count = [], 0
        while True:
            check()
            chunk = os.read(descriptor, min(65536, limits["receiptBytes"] + 1 - count))
            if not chunk:
                break
            count += len(chunk)
            if count > limits["receiptBytes"] or count > before.st_size:
                raise ValueError("receipt grew")
            chunks.append(chunk)
        if count != before.st_size or stamp(os.fstat(descriptor)) != stamp(before) or stamp(os.lstat(path)) != stamp(before):
            raise ValueError("receipt changed")
        data = b"".join(chunks)
        return json.loads(data.decode("utf-8"), object_pairs_hook=pairs), hashlib.sha256(data).hexdigest(), stamp(before)
    finally:
        os.close(descriptor)

root_facts, container_facts = os.lstat(root), os.lstat(container)
if not stat.S_ISDIR(root_facts.st_mode) or not stat.S_ISDIR(container_facts.st_mode) or root_facts.st_dev != container_facts.st_dev:
    raise ValueError("ordinary same-device reference root and namespace")
ancestor = root.parent
while True:
    if not stat.S_ISDIR(os.lstat(ancestor).st_mode):
        raise ValueError("ordinary nofollow census ancestry")
    if ancestor.parent == ancestor:
        break
    ancestor = ancestor.parent
before_root = stamp(root_facts)
before_container = stamp(container_facts)
names = sorted(os.listdir(container))
if len(names) > limits["generations"]:
    raise ValueError("generation roster capacity")
for prefix in names:
    check()
    if not re.fullmatch("[a-f0-9]{20}", prefix):
        errors.append({"prefix":prefix,"reason":"unknown-generation-role"})
        continue
    directory = container / prefix
    facts = os.lstat(directory)
    if not stat.S_ISDIR(facts.st_mode) or facts.st_dev != os.lstat(container).st_dev:
        errors.append({"prefix":prefix,"reason":"foreign-generation-directory"})
        continue
    path = directory / "bindings.json"
    try:
        value, digest, identity = read_receipt(path)
        expected = {"version","purpose","scope","generation","ownerRelationSha256","groupSha256","recipeSha256","members","bindings","ownerReferences"}
        if set(value) != expected or value["version"] != 3 or value["purpose"] != "current-dependency-source-owner-ready":
            raise ValueError("unqualified closed receipt format")
        generation = value["generation"]
        if not isinstance(generation,str) or not re.fullmatch("[a-f0-9]{64}", generation) or generation[:20] != prefix:
            raise ValueError("exact full generation")
        if not isinstance(value["ownerReferences"],list) or len(value["ownerReferences"]) > 63 or not isinstance(value["bindings"],list) or not 0 < len(value["bindings"]) <= 4096:
            raise ValueError("bounded reference and Source arrays")
        references = []
        for reference in value["ownerReferences"]:
            if set(reference) != {"scope","generation"} or not isinstance(reference["generation"],str) or not re.fullmatch("[a-f0-9]{64}",reference["generation"]):
                raise ValueError("closed owner reference")
            references.append(reference["generation"])
        sources = []
        for binding in value["bindings"]:
            if set(binding) != {"sourceDirectory","targetDirectory"} or any(not isinstance(binding[key],str) or len(binding[key].encode("utf-16-le"))//2 > 256 for key in binding):
                raise ValueError("closed bounded Source binding")
            sources.append(binding["sourceDirectory"])
        rows.append({"prefix":prefix,"generation":generation,"scope":value["scope"],"recipeSha256":value["recipeSha256"],"receiptSha256":digest,"receiptStamp":identity,"rootStamp":stamp(facts),"references":sorted(set(references)),"sources":sorted(set(sources))})
    except Exception as error:
        errors.append({"prefix":prefix,"reason":type(error).__name__+": "+str(error)[:200]})
    if len(rows) % 256 == 0:
        print("[DEBUG] retained reference census "+str(len(rows)),file=sys.stderr)

by_generation = {row["generation"]:row for row in rows}
graph = {key:[edge for edge in row["references"] if edge in by_generation] for key,row in by_generation.items()}
unknown = [{"source":key,"target":edge} for key,row in by_generation.items() for edge in row["references"] if edge not in by_generation]
visited, order = set(), []
for start in sorted(graph):
    if start in visited:
        continue
    visited.add(start)
    stack = [(start,0)]
    while stack:
        check()
        node, at = stack[-1]
        if at < len(graph[node]):
            edge = graph[node][at]
            stack[-1] = (node,at+1)
            if edge not in visited:
                visited.add(edge)
                stack.append((edge,0))
        else:
            order.append(node)
            stack.pop()
reverse = {key:[] for key in graph}
for key, edges in graph.items():
    for edge in edges:
        reverse[edge].append(key)
components, assigned = [], set()
for start in reversed(order):
    if start in assigned:
        continue
    component, stack = [], [start]
    assigned.add(start)
    while stack:
        node = stack.pop()
        component.append(node)
        for edge in reverse[node]:
            if edge not in assigned:
                assigned.add(edge)
                stack.append(edge)
    if len(component) > 1 or start in graph[start]:
        components.append(sorted(component))
sources = sorted(set(source for row in rows for source in row["sources"]))
if len(sources) > limits["sources"]:
    errors.append({"reason":"aggregate Source endpoint capacity"})
else:
    for source in sources:
        check()
        path = Path(source)
        if not path.is_absolute() or os.path.normpath(source) != source or path.name != "node_modules" or os.path.commonpath([str(workspace),source]) != str(workspace):
            errors.append({"reason":"foreign Source endpoint","source":source})
            continue
        try:
            facts = os.lstat(source)
            if stat.S_ISLNK(facts.st_mode):
                raw = os.readlink(source)
                target = os.path.normpath(os.path.join(str(path.parent),raw))
                if stamp(os.lstat(source)) != stamp(facts):
                    raise ValueError("Source alias changed")
                matched = next((row["generation"] for row in rows if target == str(container/row["prefix"]/"node_modules") or target.startswith(str(container/row["prefix"]/"w")+os.sep)),None)
                aliases.append({"source":source,"kind":"link","target":target,"generation":matched,"stamp":stamp(facts)})
            else:
                aliases.append({"source":source,"kind":"directory" if stat.S_ISDIR(facts.st_mode) else "other","generation":None,"stamp":stamp(facts)})
        except FileNotFoundError:
            aliases.append({"source":source,"kind":"absent","generation":None})
        except Exception as error:
            errors.append({"reason":type(error).__name__+": "+str(error)[:200],"source":source})
for row in rows:
    if stamp(os.lstat(container/row["prefix"]/"bindings.json")) != row["receiptStamp"] or stamp(os.lstat(container/row["prefix"])) != row["rootStamp"]:
        errors.append({"prefix":row["prefix"],"reason":"terminal receipt or generation changed"})
if sorted(os.listdir(container)) != names or stamp(os.lstat(root)) != before_root or stamp(os.lstat(container)) != before_container:
    errors.append({"reason":"terminal complete physical roster changed"})
active = set(alias["generation"] for alias in aliases if alias.get("generation"))
result = {"authority":False,"purpose":"bounded-retained-reference-census","root":str(root),"limits":limits,"elapsedMilliseconds":int((time.monotonic()-started)*1000),"complete":not errors and not unknown,"receipts":rows,"errors":errors,"unknownReferences":unknown,"sourceAliases":aliases,"cycles":components,"cyclesWithoutObservedDirectSourceRoot":[component for component in components if not active.intersection(component)],"inactivityProved":False,"payloadWalked":False,"effects":0}
(generated/"dependency-reference-census.json").write_text(json.dumps(result,ensure_ascii=False,indent=2)+"\n",encoding="utf-8")
report = "# Actual Retained Reference Census\n\nRead-only DATA; no payload walk, Source readiness, pin custody or retirement authority.\n\n"+json.dumps({"complete":result["complete"],"generations":len(rows),"errors":len(errors),"unknownReferences":len(unknown),"cycles":components,"cyclesWithoutObservedDirectSourceRoot":result["cyclesWithoutObservedDirectSourceRoot"],"elapsedMilliseconds":result["elapsedMilliseconds"]},ensure_ascii=False,indent=2)+"\n\nFull bounded DATA is in generated/dependency-reference-census.json. A metadata SCC alone never authorizes deletion; all actual pins, Source/live and captured artifact references still require the genuine owner.\n"
(ticket/"dependency-reference-census.md").write_text(report,encoding="utf-8")
print("[DEBUG] "+json.dumps({"generations":len(rows),"complete":result["complete"],"cycles":len(components),"errors":len(errors),"report":str(ticket/"dependency-reference-census.md")},ensure_ascii=False))
