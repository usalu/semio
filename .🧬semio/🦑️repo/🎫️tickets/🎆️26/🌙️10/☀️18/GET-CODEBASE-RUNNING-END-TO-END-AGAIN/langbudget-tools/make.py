#!/usr/bin/env python3
"""🧮️ Builds the staged batches of lane langbudget from the CURRENT live files (loader) and from the nolegacy `lang` batch (lang): the captured-command loader stops inlining the refusal catalog.
usage: python3 -I make.py [--root DIR]   (default root: the live checkout this file sits in; writes staged/loader and staged/lang beside this file)
loader : edits on live; base = live bytes. lang: nolegacy lang copied, the two files this lane edits (closure loader, sites catalog) rebased onto loader's new bytes, 4 pre-authentication catalog messages become machine tokens."""
import hashlib, json, os, shutil, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tests_edit

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.path.abspath(os.path.join(C, "../../../../../../.."))
NL = os.path.join(C, "nolegacy-tools", "staged", "lang")
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
BUN = LIB + "🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts"
SITES = LIB + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts"
SCHEMA = LIB + "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json"
IDENT = LIB + "🏗️build/📥️source/🔏️closure/🚪️command/🧪️tests/🏷️loader-identity/🟦️.ts"
CSITES = LIB + "🏗️build/📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts"
TITLES = {
    "loader": "\U0001F9EE\uFE0F Staged batch `loader` (lane langbudget): the captured-command stream loader no longer inlines the refusal catalog, so its `--eval` argument stays bounded below the 32767-character Windows command line however the catalog grows; texts come from the authenticated snapshot. Apply before nolegacy `lang` (langbudget `lang`).",
    "lang": "\U0001F5E3\uFE0F Staged batch `lang` (lane langbudget, re-cut of nolegacy `lang`): the repository language law to 0 findings on top of the bounded loader; the four refusals raised before a snapshot is authenticated are machine tokens, not catalog messages. Apply after langbudget `loader` and nolegacy `lease` + `nobaseline`.",
}
PRE_AUTH = ["digest-required", "snapshot-bounded", "snapshot-changed", "snapshot-shortened"]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as f:
        return f.read()


def edit(text, old, new, count=1):
    if text.count(old) != count:
        sys.exit("anchor count %d != %d: %r" % (text.count(old), count, old[:90]))
    return text.replace(old, new)


def bun_edit(text, lang):
    text = edit(text, "import{closureRefusalMessagesV1,closureRefusalTextV1,closureTargetSchemesV1" + (",type ClosureRefusalCodeV1" if lang else "") + "}from",
                "import{closureRefusalCatalogPathV1,closureRefusalCatalogTableV1,closureRefusalTextV1,closureTargetSchemesV1" + (",type ClosureRefusalCodeV1" if lang else "") + "}from")
    early = ("closureRefusalTextV1=(${closureRefusalTextV1.toString()}),refusalMessages=${JSON.stringify(closureRefusalMessagesV1)},refusal=(code,fields)=>Error(closureRefusalTextV1(refusalMessages,code,fields,process.env));"
             if lang else None)
    new_early = ("const closureRefusalTextV1=(${closureRefusalTextV1.toString()}),closureRefusalCatalogTableV1=(${closureRefusalCatalogTableV1.toString()}),refusalCatalog={table:undefined,read:()=>undefined},"
                 "refusal=(code,fields)=>Error(closureRefusalTextV1(refusalCatalog.read()??{},code,fields,process.env));")
    if lang:
        text = edit(text, "const " + early, new_early)
    else:
        text = edit(text, "\n${acquisition}\n", "\n" + new_early + "\n${acquisition}\n")
        text = edit(text, "closureRefusalTextV1=(${closureRefusalTextV1.toString()}),refusalMessages=${JSON.stringify(closureRefusalMessagesV1)},", "")
        text = edit(text, "refusal=(code,fields)=>Error(closureRefusalTextV1(refusalMessages,code,fields,process.env)),", "")
    catalog = ("const catalogAt=value.files.findIndex(row=>row&&row.path===${JSON.stringify(closureRefusalCatalogPathV1)}),catalogEager=catalogAt<0||spans?undefined:value.files[catalogAt].base64;"
               "refusalCatalog.read=()=>refusalCatalog.table??=catalogAt<0?undefined:closureRefusalCatalogTableV1(spans?encoded.toString('latin1',spans[catalogAt][0],spans[catalogAt][1]):catalogEager,value.files[catalogAt].sha256,hash);")
    return edit(text, "const{images,resolutions}=admitCapturedImagesV1(value,encoded,spans,", catalog + "const{images,resolutions}=admitCapturedImagesV1(value,encoded,spans,")


def sites_edit(text):
    text = edit(text, "const policy=schema.$defs.Policy.const,messages=schema.$defs.Messages.const;", "const policy=schema.$defs.Policy.const,messages=schema.$defs.Messages.const;")
    return edit(text, "/** 🔖️ The target schemes of derived import-map entries. */",
                "/** 📍️ Where the refusal catalog sits, relative to the repository root: a snapshot that carries this file renders every refusal after its authentication from it, so a loader never inlines the catalog and its command line stays bounded however the catalog grows; only the refusals raised while the snapshot itself is read, before anything is authenticated, are machine tokens. */\n"
                "export const closureRefusalCatalogPathV1=policy.catalogPath;\n"
                "/** 🗃️ Reads the refusal table out of the base64 bytes of the catalog file as a snapshot row carries them, once their declared hash holds; undefined for anything else, so the refusal then names its code and fields; self-contained so loaders can embed its source. */\n"
                "export function closureRefusalCatalogTableV1(encoded:string|undefined,sha256:string|undefined,hash:(bytes:Uint8Array)=>string):Readonly<Record<string,Readonly<Record<string,Readonly<{refusal:string;fix:string}>>>>>|undefined{\n"
                " try{const bytes=Buffer.from(encoded??\"\",\"base64\");if(!encoded||bytes.toString(\"base64\")!==encoded||hash(bytes)!==sha256)return undefined;const table=JSON.parse(bytes.toString(\"utf8\"))?.$defs?.Messages?.const;return table&&typeof table===\"object\"?table:undefined;}catch{return undefined;}\n"
                "}\n"
                "/** 🔖️ The target schemes of derived import-map entries. */")


def schema_edit(text, lang):
    text = edit(text, '    "rootScheme": "semio-root:",\n', '    "rootScheme": "semio-root:",\n    "catalogPath": ' + json.dumps(SCHEMA, ensure_ascii=False) + ',\n')
    if lang:
        d = json.loads(text)
        for code in PRE_AUTH:
            del d["$defs"]["Messages"]["const"][code]
        text = json.dumps(d, ensure_ascii=False, indent=1) + "\n"
    return text


def write_batch(name, rows, title):
    out = os.path.join(HERE, "staged", name)
    for sub in ("files", "base"):
        shutil.rmtree(os.path.join(out, sub), ignore_errors=True)
        os.makedirs(os.path.join(out, sub))
    manifest = {}
    for index, (rel, base, new) in enumerate(rows):
        ext = os.path.splitext(rel)[1]
        fname = "%03d%s" % (index + 1, ext)
        if base is not None:
            open(os.path.join(out, "base", fname), "wb").write(base)
        open(os.path.join(out, "files", fname), "wb").write(new)
        manifest[rel] = {"file": fname, "base": sha(base) if base is not None else None, "new": sha(new)}
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(HERE, "stage.template.py"), encoding="utf8").read().replace("{TITLE}", title)
    open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(template)
    os.chmod(os.path.join(out, "stage.py"), 0o755)
    return out, manifest


def main():
    rows = {}
    for rel, edit_fn in ((BUN, lambda t: bun_edit(t, False)), (SITES, sites_edit), (SCHEMA, lambda t: schema_edit(t, False)), *tests_edit.EDITS):
        base = read(os.path.join(ROOT, rel))
        rows[rel] = (base, edit_fn(base.decode("utf8")).encode("utf8"))
    extra = json.load(open(os.path.join(HERE, "extra-loader.json"), encoding="utf8")) if os.path.exists(os.path.join(HERE, "extra-loader.json")) else {}
    loader_rows = [(rel, b, n) for rel, (b, n) in rows.items()]
    for rel, spec in extra.items():
        base = read(os.path.join(ROOT, rel)) if os.path.exists(os.path.join(ROOT, rel)) else None
        loader_rows.append((rel, base, read(os.path.join(HERE, spec)) if isinstance(spec, str) else None))
    out, manifest = write_batch("loader", loader_rows, TITLES["loader"])
    print("loader", len(manifest), "rows ->", out)

    lang = json.load(open(os.path.join(NL, "manifest.json"), encoding="utf8"))
    lrows = []
    for rel, row in lang.items():
        newb = read(os.path.join(NL, "files", row["file"]))
        baseb = read(os.path.join(NL, "base", row["file"])) if row["base"] is not None else None
        if rel == BUN:
            assert sha(baseb) == sha(rows[BUN][0]), "lang base of the loader is not live"
            baseb, newb = rows[BUN][1], bun_edit(newb.decode("utf8"), True).encode("utf8")
        elif rel == SCHEMA:
            assert sha(baseb) == sha(rows[SCHEMA][0]), "lang base of the catalog is not live"
            baseb, newb = rows[SCHEMA][1], schema_edit(newb.decode("utf8"), True).encode("utf8")
        else:
            live = read(os.path.join(ROOT, rel)) if os.path.exists(os.path.join(ROOT, rel)) else None
            assert (live is None and baseb is None) or (live is not None and sha(live) == sha(baseb)), "lang base drifted: " + rel
        lrows.append((rel, baseb, newb))
    index = {rel: i for i, (rel, _b, _n) in enumerate(lrows)}
    for rel, edit_fn in tests_edit.LANG_EDITS:
        if rel in index:
            i = index[rel]
            lrows[i] = (rel, lrows[i][1], edit_fn(lrows[i][2].decode("utf8")).encode("utf8"))
        else:
            base = rows[rel][1] if rel in rows else read(os.path.join(ROOT, rel))
            lrows.append((rel, base, edit_fn(base.decode("utf8")).encode("utf8")))
    out, manifest = write_batch("lang", lrows, TITLES["lang"])
    print("lang", len(manifest), "rows ->", out)


main()
