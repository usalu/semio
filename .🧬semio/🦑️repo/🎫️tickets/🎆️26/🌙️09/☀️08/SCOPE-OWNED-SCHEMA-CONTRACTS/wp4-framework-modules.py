#!/usr/bin/env python3
"""🧬 WP4 framework-module schema consolidation.

Moves every non-canonical framework-module schema file into its owner scope's
`🧬️schema/🔣️.json` module, one module per scope, exports as named `$defs`.
Writes `wp4-framework-modules.map.json` (old path/-id -> new path/-pointer) for
the consumer-rewrite pass.
"""
import json, os, re, subprocess, shutil, sys
from collections import OrderedDict, defaultdict

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)
MODROOT = "🧰️framework/🔨️modules/"
DRAFT = "http://json-schema.org/draft-07/schema#"
EMO = re.compile(r"^[^\x00-\x7f]+")

DRY = "--dry" in sys.argv
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS"


def at(s):
    return EMO.sub("", s).lstrip("️")


def pas(s):
    s = at(s)
    s = re.sub(r"\.(schema|json)$", "", s)
    return "".join(p[:1].upper() + p[1:] for p in re.split(r"[-_.\s]+", s) if p)


# owner hoists: 🧱️elements/* is not an eligible scope owner level (contract §A)
MODULE_HOIST = {
    MODROOT + "🖱️ui/🧱️elements/📨️UIDialog": MODROOT + "🖱️ui",
}

FIXDIRS = {
    "🧪️fixtures",
    "🧫️fixtures",
    "🧪️tests",
    "🧪️fixture",
    "🧫️fixture",
    "🧪️conformance",
    "🧬️contracts",
    "📚️examples",
}


def scope_path(module):
    rel = module[len(MODROOT):]
    return "framework/" + "/".join(at(s) for s in rel.split("/"))


def module_id(module):
    return f"https://semio.tech/schema/{scope_path(module)}/schema.json"


def scope_id(module):
    return scope_path(module).replace("/", ".")


EXPORT_OVERRIDE = {
    MODROOT + "🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧬️schema/🔣️.schema.json": "ObjcRuntimeAbiFixture",
    MODROOT + "🌱️value/🗂️ordered/🧺️set/🧬️schema/🔣️.schema.json": "OrderedSet",
    MODROOT + "🎭️actor/🎠️activation/🧬️schema.json": "ActivationReservation",
    MODROOT + "⏱️trace/⏱️clock/🧬️contention/🔣️.schema.json": "Contention",
    MODROOT + "🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🔣️.schema.json": "TypedScene",
    MODROOT + "⏳️async/🔐️use/🧪️fixtures/🧬️.schema.json": "UseFixture",
    MODROOT + "⏳️async/🔔️deferred-wake/🧪️fixtures/🧬️.schema.json": "DeferredWakeFixture",
    MODROOT + "⏳️async/🔔️maintenance/🧪️fixtures/🧬️.schema.json": "MaintenanceFixture",
    MODROOT + "🌱️value/🔁️codec/🧪️fixtures/🧬️.schema.json": "CodecFixture",
    MODROOT + "🌱️value/🗂️ordered/🧫️fixtures/🧬️.schema.json": "OrderedMapFixture",
    MODROOT + "🎯️action-bus/🧹️wire-retirement/🧪️fixture/🧬️.schema.json": "WireRetirementFixture",
    MODROOT + "🖱️ui/🧬️contract/🧪️fixtures/🔣️.schema.json": "ContractFixture",
    MODROOT + "🎭️actor/🚪️lifetime/🧯️fault.schema.json": "CloseFaultFixture",
    MODROOT + "🖼️assets/🌱️metabolism/🎨️representation/🧬️catalog.schema.json": "RepresentationCatalog",
    MODROOT + "🖼️assets/🔤️fonts/🧬️catalog.schema.json": "FontCatalog",
    MODROOT + "🖼️assets/🥽️mesh/🧬️catalog.schema.json": "MeshCatalog",
    MODROOT + "🖼️assets/🔍️resolver/🧬️delivery.schema.json": "Delivery",
    MODROOT + "🕸️graph/🛂️manifest/🧬️outputs.schema.json": "Outputs",
    MODROOT + "🖱️ui/🎨️styling/🧬️favicon.schema.json": "Favicon",
    MODROOT + "🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️components.schema.json": "Components",
    MODROOT + "🖱️ui/🧬️contract/📚️examples/🧪️conformance/🧬️catalog.schema.json": "ConformanceCatalogFixture",
}


def classify(f):
    """-> (module_dir, export_id, role)"""
    segs = f.split("/")
    name = segs[-1]
    if "🧬️schema" in segs[:-1]:                      # already inside a module dir
        i = segs.index("🧬️schema")
        mod = "/".join(segs[:i])
        return MODULE_HOIST.get(mod, mod), pas(segs[i - 1]), "inplace"
    fx = [i for i, s in enumerate(segs[:-1]) if s in FIXDIRS]
    if fx:
        oi = min(fx)
        mod = "/".join(segs[:oi])
        tail = segs[oi + 1: -1]
        base = "".join(pas(t) for t in tail) or pas(name)
        if mod in MODULE_HOIST:
            base = pas(segs[oi - 1]) + base
            mod = MODULE_HOIST[mod]
        return mod, (base if base.endswith("Fixture") else base + "Fixture"), "fixture"
    if segs[-2] in ("📐️schema", "📐️fixture-schema"):
        mod = "/".join(segs[:-2])
        return MODULE_HOIST.get(mod, mod), pas(segs[-3]) + "Fixture", "oracle"
    if segs[-2] == "🛂️schema":
        mod = "/".join(segs[:-2])
        return MODULE_HOIST.get(mod, mod), pas(segs[-3]), "contract"
    mod = "/".join(segs[:-1])
    exp = pas(segs[-2]) if name == "🧬️schema.json" else pas(name)
    return MODULE_HOIST.get(mod, mod), exp, "contract"


def polish(f, exp):
    if f in EXPORT_OVERRIDE:
        return EXPORT_OVERRIDE[f]
    e = re.sub(r"(?<=.)Schema(?=Fixture$|$)", "", exp)
    return e or exp


def collect():
    files = subprocess.check_output(["git", "ls-files", "🧰️framework/🔨️modules"]).decode().splitlines()
    viol = [
        f for f in files
        if "/🧬️mutations/" not in f
        and not f.startswith(MODROOT + "🧬️schema/")
        and (f.endswith(".schema.json") or f.endswith("/🧬️schema.json")
             or "/📐️schema/" in f or "/🛂️schema/" in f
             or "/📐️fixture-schema/" in f or "/🧬️contracts/" in f)
    ]
    # data fixtures, not schemas
    viol = [f for f in viol if not f.endswith("🧬️contracts/♿️modal/🔣️.json")]
    return files, viol


def load(p):
    with open(p, encoding="utf-8") as fh:
        return json.load(fh)


def retarget_refs(node, renames):
    """Rewrite local `#/definitions/<old>` pointers after a collision rename."""
    if isinstance(node, list):
        return [retarget_refs(x, renames) for x in node]
    if not isinstance(node, dict):
        return node
    out = OrderedDict()
    for k, v in node.items():
        if k == "$ref" and isinstance(v, str):
            for old, new in renames.items():
                p = "#/" + old if old.startswith("$defs/") else "#/definitions/" + old
                if v == p or v.startswith(p + "/"):
                    v = "#/definitions/" + new + v[len(p):]
                    break
        out[k] = retarget_refs(v, renames)
    return out


def hoist(body, exp, sink_defs, sink_definitions, renamed):
    """Strip meta keys, hoist local definitions/$defs to the module root."""
    b = dict(body)
    for k in ("$schema", "$id", "id"):
        b.pop(k, None)
    local = {}
    for n, v in (b.pop("definitions", {}) or {}).items():
        n2 = n
        if n in sink_definitions and sink_definitions[n] != v:
            n2 = exp[:1].lower() + exp[1:] + n[:1].upper() + n[1:]
            local[n] = n2
        sink_definitions[n2] = v
    inner = b.pop("$defs", {}) or {}
    for n, v in inner.items():
        n2 = n
        if n in sink_definitions and sink_definitions[n] != v:
            n2 = exp[:1].lower() + exp[1:] + n[:1].upper() + n[1:]
        local["$defs/" + n] = n2
        sink_definitions[n2] = v
    if local:
        b = retarget_refs(b, local)
        for old, new in local.items():
            sink_definitions[new] = retarget_refs(sink_definitions[new], local)
    return b, local


def to_draft07(node):
    """2020-12 -> draft-07 (prefixItems/items:false, unevaluated*)."""
    if isinstance(node, list):
        return [to_draft07(x) for x in node]
    if not isinstance(node, dict):
        return node
    out = OrderedDict()
    pre = node.get("prefixItems")
    for k, v in node.items():
        if k in ("unevaluatedProperties", "unevaluatedItems"):
            continue
        if k == "prefixItems":
            out["items"] = [to_draft07(x) for x in v]
            continue
        if k == "items" and pre is not None:
            out["additionalItems"] = False if v is False else to_draft07(v)
            continue
        out[k] = to_draft07(v)
    return out


# ── shared shapes owned by framework.ui (contract §D: retained-command-limits /
# ── -routes are per-subset contracts; the *shape* is exported here).
UI_SHARED_DEFINITIONS = OrderedDict([
    ("retainedCommandLane", {"enum": ["Artifact", "Config", "HostOnly", "artifact", "config", "host-only", "hostOnly"]}),
    ("retainedCommandByteBudget", {
        "type": "object", "additionalProperties": False,
        "required": ["rawBytes", "checkpointBytes", "configValueBytes", "configBaseBytes",
                     "commandStepBytes", "storeStepBytes", "workItems", "preparationPhases"],
        "properties": {k: {"type": "integer", "minimum": 0} for k in
                       ["rawBytes", "checkpointBytes", "configValueBytes", "configBaseBytes",
                        "commandStepBytes", "storeStepBytes", "workItems", "preparationPhases"]}}),
    ("retainedCommandStepBudget", {
        "type": "object", "additionalProperties": False,
        "required": ["maxRawBytes", "maxDecodedItems", "maxWorkUnitsPerStep", "maxOutputBytes", "maxStepMicros"],
        "properties": {k: {"type": "integer", "minimum": 0} for k in
                       ["maxRawBytes", "maxDecodedItems", "maxWorkUnitsPerStep", "maxOutputBytes",
                        "maxStepMicros", "maxCheckpointBytes", "maxInFlightPages"]}}),
    ("retainedCommandClassBudget", {
        "type": "object", "additionalProperties": False, "required": ["bounded"],
        "properties": {"bounded": {"$ref": "#/definitions/retainedCommandStepBudget"},
                       "resumable": {"$ref": "#/definitions/retainedCommandStepBudget"}}}),
    ("retainedCommandPublicationContract", {
        "type": "object", "additionalProperties": False, "required": ["toolId", "lanes"],
        "properties": {"toolId": {"type": "string", "minLength": 1},
                       "lanes": {"type": "array", "minItems": 1,
                                 "items": {"$ref": "#/definitions/retainedCommandLane"}}}}),
    ("retainedCommandBoundaryCase", {
        "type": "object", "additionalProperties": False, "required": ["name", "bytes", "accepted"],
        "properties": {"name": {"type": "string", "minLength": 1},
                       "bytes": {"type": "integer", "minimum": 0},
                       "accepted": {"type": "boolean"}}}),
    ("retainedCommandOracle", {
        "type": "object", "additionalProperties": False,
        "required": ["library", "scope", "runtimeDependency"],
        "properties": {"library": {"type": "string", "minLength": 1},
                       "scope": {"type": "string", "minLength": 1},
                       "runtimeDependency": {"type": "boolean"},
                       "ownedInterface": {"type": "string", "minLength": 1},
                       "expected": {"type": "object",
                                    "additionalProperties": {"anyOf": [{"type": "integer"}, {"type": "boolean"}]}}}}),
    ("retainedCommandRouteDisposition", {
        "type": "object", "additionalProperties": False,
        "required": ["id", "disposition", "lanes"],
        "properties": {"id": {"type": "string", "minLength": 1},
                       "disposition": {"enum": ["Migrated", "BatchOnlyPendingRewrite",
                                                "migrated", "batch-only-pending-rewrite"]},
                       "lanes": {"type": "array", "items": {"$ref": "#/definitions/retainedCommandLane"}},
                       "blocker": {"type": "string"}, "reason": {"type": "string"}}}),
    ("retainedCommandRouteExecutionLane", {
        "type": "object", "additionalProperties": False,
        "required": ["id", "execution", "lanes", "reason"],
        "properties": {"id": {"type": "string", "minLength": 1},
                       "execution": {"enum": ["bounded-first-step", "bounded", "batch", "resumable"]},
                       "lanes": {"type": "array", "minItems": 1,
                                 "items": {"$ref": "#/definitions/retainedCommandLane"}},
                       "reason": {"type": "string", "minLength": 1}}}),
    ("retainedCommandRouteExecutionFeature", {
        "type": "object", "additionalProperties": False,
        "required": ["id", "execution", "feature"],
        "properties": {"id": {"type": "string", "minLength": 1},
                       "execution": {"enum": ["bounded-first-step", "bounded", "batch", "resumable"]},
                       "admission": {"enum": ["failClosed", "migrated"]},
                       "status": {"enum": ["batch-only", "migrated"]},
                       "feature": {"type": "string", "minLength": 1}}}),
    ("retainedCommandRoute", {"oneOf": [
        {"$ref": "#/definitions/retainedCommandRouteDisposition"},
        {"$ref": "#/definitions/retainedCommandRouteExecutionLane"},
        {"$ref": "#/definitions/retainedCommandRouteExecutionFeature"}]}),
    ("retainedCommandCorpusLimits", {
        "type": "object", "additionalProperties": False,
        "required": ["rejectedAdditionalBytes", "expectedWorkItems"],
        "properties": {"schema": {"type": "string", "minLength": 1},
                       "maximumTextBytes": {"type": "integer", "minimum": 0},
                       "maximumSchemaBytes": {"type": "integer", "minimum": 0},
                       "maximumRawBytes": {"type": "integer", "minimum": 0},
                       "maximumWorkItems": {"type": "integer", "minimum": 0},
                       "rejectedAdditionalBytes": {"type": "integer", "minimum": 0},
                       "expectedWorkItems": {"type": "integer", "minimum": 0},
                       "toolIds": {"type": "array", "items": {"type": "string", "minLength": 1}}}}),
    ("retainedCommandDeclaredLimits", {
        "type": "object", "additionalProperties": False, "required": ["controller", "documentSchema", "factory", "limits"],
        "properties": {"version": {"type": "integer", "minimum": 1},
                       "schema": {"type": "string", "minLength": 1},
                       "owner": {"type": "string", "minLength": 1},
                       "controller": {"type": "string", "minLength": 1},
                       "documentSchema": {"type": "string", "minLength": 1},
                       "factory": {"type": "string", "minLength": 1},
                       "limits": {"oneOf": [{"$ref": "#/definitions/retainedCommandByteBudget"},
                                            {"$ref": "#/definitions/retainedCommandClassBudget"}]},
                       "publicationContracts": {"type": "array", "items": {"$ref": "#/definitions/retainedCommandPublicationContract"}},
                       "routes": {"$ref": "#/$defs/RetainedCommandRoutes"},
                       "boundaryCases": {"type": "array", "items": {"$ref": "#/definitions/retainedCommandBoundaryCase"}},
                       "oracle": {"$ref": "#/definitions/retainedCommandOracle"}}}),
])
UI_SHARED_DEFS = OrderedDict([
    ("RetainedCommandLimits", {
        "$comment": "Shared shape for every artifact-subset 🧫️retained-command-limits fixture; owners narrow it with const values.",
        "oneOf": [{"$ref": "#/definitions/retainedCommandDeclaredLimits"},
                  {"$ref": "#/definitions/retainedCommandCorpusLimits"}]}),
    ("RetainedCommandRoutes", {
        "$comment": "Shared shape for retained-command route tables, standalone or nested in RetainedCommandLimits.",
        "type": "array", "items": {"$ref": "#/definitions/retainedCommandRoute"}}),
    ("RetainedCommandRoutesDocument", {
        "type": "object", "additionalProperties": False, "required": ["schema", "routes"],
        "properties": {"schema": {"type": "string", "minLength": 1},
                       "maximumRawBytes": {"type": "integer", "minimum": 0},
                       "maximumWorkItems": {"type": "integer", "minimum": 0},
                       "routes": {"$ref": "#/$defs/RetainedCommandRoutes"}}}),
])
UI_MODULE = MODROOT + "🖱️ui"


def main():
    files, viol = collect()
    fset = set(files)
    groups = defaultdict(list)
    for f in sorted(viol):
        mod, exp, role = classify(f)
        groups[mod].append(dict(src=f, exp=polish(f, exp), role=role))

    # dedupe export names per module
    for mod, rows in groups.items():
        seen = set()
        for r in rows:
            e, n, k = r["exp"], r["exp"], 2
            while n in seen:
                n = e + str(k)
                k += 1
            seen.add(n)
            r["exp"] = n

    mapping = {"modules": {}, "idRewrites": {}, "pathRewrites": {}, "deleted": []}

    for mod in sorted(groups):
        rows = groups[mod]
        target_dir = os.path.join(mod, "🧬️schema")
        target = os.path.join(target_dir, "🔣️.json")
        mid = module_id(mod)
        defs, definitions = OrderedDict(), OrderedDict()
        old_ids, defrenames = {}, {}

        # fold an already-present module file in first
        if target in fset and not any(r["src"] == target for r in rows):
            cur = load(target)
            if isinstance(cur, dict) and "$defs" in cur and str(cur.get("$id", "")).startswith("https://semio.tech/schema/framework/"):
                defs.update(cur["$defs"])
                definitions.update(cur.get("definitions", {}) or {})
            else:
                exp = pas(mod.split("/")[-1])
                if cur.get("$id"):
                    old_ids[cur["$id"]] = exp
                defs[exp], ren = hoist(to_draft07(cur), exp, defs, definitions, None)
                defrenames.update({exp: ren} if ren else {})

        for r in rows:
            if r["src"] == target:
                continue
            body = load(r["src"])
            if body.get("$id"):
                old_ids[body["$id"]] = r["exp"]
            defs[r["exp"]], ren = hoist(to_draft07(body), r["exp"], defs, definitions, None)
            if ren:
                defrenames[r["exp"]] = ren

        if mod == UI_MODULE:
            for n, v in UI_SHARED_DEFINITIONS.items():
                definitions[n] = v
            for n, v in UI_SHARED_DEFS.items():
                defs[n] = v

        doc = OrderedDict()
        doc["$schema"] = DRAFT
        doc["$id"] = mid
        doc["title"] = " ".join(
            w.capitalize() for w in scope_path(mod).split("/")[1:]
        ) + " Schema Module"
        if definitions:
            doc["definitions"] = definitions
        doc["$defs"] = defs
        if len(defs) == 1:
            doc["allOf"] = [{"$ref": "#/$defs/" + next(iter(defs))}]

        if not DRY:
            os.makedirs(target_dir, exist_ok=True)
            with open(target, "w", encoding="utf-8") as fh:
                json.dump(doc, fh, ensure_ascii=False, indent=2)
                fh.write("\n")

        mapping["modules"][scope_id(mod)] = dict(
            path=target, id=mid, exports=list(defs.keys()),
            sources=[r["src"] for r in rows],
            definitionRenames=defrenames,
        )
        for oid, exp in old_ids.items():
            mapping["idRewrites"][oid] = dict(id=mid, export=exp)
        for r in rows:
            if r["src"] != target:
                mapping["pathRewrites"][r["src"]] = target
                mapping["deleted"].append(r["src"])

    # remove superseded files + now-empty directories
    for p in ([] if DRY else mapping["deleted"]):
        if os.path.exists(p):
            os.remove(p)
    for p in ([] if DRY else mapping["deleted"]):
        d = os.path.dirname(p)
        while d and d.startswith(MODROOT.rstrip("/")):
            try:
                if not os.listdir(d):
                    os.rmdir(d)
                    d = os.path.dirname(d)
                    continue
            except OSError:
                pass
            break

    out = os.path.join(TICKET, "wp4-framework-modules.map%s.json" % ("-dry" if DRY else ""))
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(mapping, fh, ensure_ascii=False, indent=1)
    print(f"modules={len(mapping['modules'])} deleted={len(mapping['deleted'])} idRewrites={len(mapping['idRewrites'])}")


if __name__ == "__main__":
    main()
