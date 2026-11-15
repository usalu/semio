"""🧯 Removes every `[DEBUG] hitcost` probe of the finished hitcost investigation from the build library."""
import sys, pathlib

root = pathlib.Path(sys.argv[1])
build = root / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
inputs = build / "🕸️graph/📥️inference/🧭️script-inputs"
swaps = {
    inputs / "🟨️.mjs": [
        ("  closureCountsV1.checked++;{const h=globalThis.__hc??={};h.entryChecks=(h.entryChecks??0)+1;if(!round)h.entryNoRound=(h.entryNoRound??0)+1;} // [DEBUG] hitcost\n", "  closureCountsV1.checked++;\n"),
        ("derive = () => { const t0 = performance.now(), h = globalThis.__hc ??= {}; return derive0().finally(() => { h.deriveN = (h.deriveN ?? 0) + 1; h.deriveMs = Math.round((h.deriveMs ?? 0) + performance.now() - t0); }); }, derive0 = () => observeDetachedV1(", "derive = () => observeDetachedV1("),
        ("await (async () => { const t0 = performance.now(), ok = await closureEntryHoldsAsyncV1(entry, scripts), h = globalThis.__hc ??= {}; h.holdN = (h.holdN ?? 0) + 1; h.holdMs = Math.round((h.holdMs ?? 0) + performance.now() - t0); return ok; })()", "await closureEntryHoldsAsyncV1(entry, scripts)"),
        ("    const t0=performance.now();try{return answerV1(path);}finally{const h=globalThis.__hc??={};h.answerCalls=(h.answerCalls??0)+1;h.answerMs=Math.round((h.answerMs??0)+performance.now()-t0);} // [DEBUG] hitcost\n", "    return answerV1(path);\n"),
        ("  async settle(paths, value) { const t0=performance.now();try{return await settleV1(paths, value);}finally{const h=globalThis.__hc??={};h.settleCalls=(h.settleCalls??0)+1;h.settleMs=Math.round((h.settleMs??0)+performance.now()-t0);} }, // [DEBUG] hitcost\n", "  settle(paths, value) {\n    return settleV1(paths, value);\n  },\n"),
    ],
    inputs / "💾️entries/🟨️.mjs": [],
    build / "👁️observation/🟨️.mjs": [],
}
for file, pairs in swaps.items():
    text = file.read_text(encoding="utf-8")
    for old, new in pairs:
        assert text.count(old) == 1, (str(file), old[:60])
        text = text.replace(old, new)
    lines = [line for line in text.splitlines(keepends=True) if not (line.lstrip().startswith("{const h=globalThis.__hc??={};") or (line.lstrip().startswith("if (!holds) { try { const bad = []") and "PROBE_OUT" in line))]
    text = "".join(lines)
    assert "__hc" not in text and "hitcost" not in text and "PROBE_OUT" not in text, str(file)
    file.write_text(text, encoding="utf-8")
    print("cleaned", file.relative_to(root))
