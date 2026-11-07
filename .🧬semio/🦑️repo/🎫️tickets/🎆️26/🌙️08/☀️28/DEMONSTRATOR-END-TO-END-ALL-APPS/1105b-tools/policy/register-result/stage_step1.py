import os, re, sys, json, hashlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import codemod as cm
ROOT = cm.ROOT
TICKET = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
OUT = os.path.join(TICKET, "🗑️generated", "1105b", "policy")
FLAT = os.path.join(OUT, "flat"); PRE = os.path.join(OUT, "preimage")
RET = "semio_plugin::io::RegistrationFault"
FAULT_BLOCK = """
/// ⚠️ A stdio plugin-root registration failed: one typed fault from the io composer or subset-validator registry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistrationFault {
    Composer(IoRegistryRegistrationError),
    SubsetValidator(SubsetValidatorRegistryError),
}

impl RegistrationFault {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Composer(_) => "stdio.registration.composer",
            Self::SubsetValidator(_) => "stdio.registration.subset-validator",
        }
    }
}

impl std::fmt::Display for RegistrationFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Composer(error) => write!(f, "{} {error}", self.code()),
            Self::SubsetValidator(error) => write!(f, "{} {error}", self.code()),
        }
    }
}

impl std::error::Error for RegistrationFault {}

impl From<IoRegistryRegistrationError> for RegistrationFault {
    fn from(error: IoRegistryRegistrationError) -> Self { Self::Composer(error) }
}

impl From<SubsetValidatorRegistryError> for RegistrationFault {
    fn from(error: SubsetValidatorRegistryError) -> Self { Self::SubsetValidator(error) }
}
"""

def one(text, old, new, rel):
    n = text.count(old)
    if n != 1: raise SystemExit(f"FAIL-CLOSED anchor count {n} in {rel}: {old[:90]!r}")
    return text.replace(old, new)

def fn_span(lines, start):
    e = cm.find_fn_end(lines, start)
    s = start
    while s > 0 and (lines[s-1].lstrip().startswith("#[") or lines[s-1].lstrip().startswith("//")):
        s -= 1
    return s, e

def read(rel): return open(os.path.join(ROOT, rel), encoding="utf-8").read()

def special(edits_out):
    # 1. framework: RegistrationFault after the io module's last item
    rel = "🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs"
    t = read(rel)
    if "pub enum RegistrationFault" in t: raise SystemExit("FAIL-CLOSED: RegistrationFault already present")
    edits_out[rel] = t.rstrip("\n") + "\n" + FAULT_BLOCK

    # 2. SemioConversion.register field type
    rel = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🦀️.rs"
    t = read(rel)
    t = one(t, "    pub register: fn(),", f"    pub register: fn() -> Result<(), {RET}>,", rel)
    edits_out[rel] = t

    # 3. stdio semio root: signature, calls, fn-pointer loop, Ok(())
    rel = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs"
    lines = read(rel).split("\n")
    sig = [i for i, l in enumerate(lines) if l.startswith("pub fn register(conversions: &[semio_s_artifact_stdio_semio_base::SemioConversion]) {")]
    if len(sig) != 1: raise SystemExit(f"FAIL-CLOSED root sig count {len(sig)}")
    s = sig[0]; e = cm.find_fn_end(lines, s)
    out = list(lines)
    out[s] = lines[s].replace(") {", f") -> Result<(), {RET}> {{")
    loop_calls = 0; call_n = 0
    for k in range(s + 1, e):
        L = lines[k]
        if L.strip() == "(conversion.register)();":
            out[k] = L.replace("(conversion.register)();", "(conversion.register)()?;"); loop_calls += 1
        elif cm.CALL_RE.match(L):
            out[k] = L.replace("register();", "register()?;"); call_n += 1
    if loop_calls != 1 or call_n != 19: raise SystemExit(f"FAIL-CLOSED root counts loop={loop_calls} calls={call_n}")
    out[e] = "    Ok(())\n" + lines[e]
    edits_out[rel] = "\n".join(out)

    # 4. dead items: remove the two definition-only functions with their attributes
    for rel, name in [("🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🦀️.rs", "reclaim_orphaned_registry_slots"),
                      ("🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/➡️sweep/🧮️sweep-machinery/🦀️.rs", "make_seed_vertex")]:
        lines = read(rel).split("\n")
        idx = [i for i, l in enumerate(lines) if re.search(rf"\bfn {name}\(", l)]
        if len(idx) != 1: raise SystemExit(f"FAIL-CLOSED dead item {name} count {len(idx)}")
        s, e = fn_span(lines, idx[0])
        # drop the blank line after the removed block to keep spacing
        if e + 1 < len(lines) and lines[e+1].strip() == "": e += 1
        edits_out[rel] = "\n".join(lines[:s] + lines[e+1:])

    # 5. raster + remodel helpers (production): OnceLock<Result> + map_err at Result<_, String> callers
    for rel in ["🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🌉️semio-bridge/🦀️.rs",
                "📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]:
        full = "✏️s/🔌️plugins/" + rel
        lines = read(full).split("\n")
        hs = [i for i, l in enumerate(lines) if l.startswith("fn ensure_stdio_semio_and_png_registered() {")]
        if len(hs) != 1: raise SystemExit(f"FAIL-CLOSED helper count {len(hs)} in {full}")
        hstart = hs[0]; hend = cm.find_fn_end(lines, hstart)
        body = lines[hstart+1:hend]
        reg = [b for b in body if "semio_s_artifact_stdio_semio::register(&[" in b]
        png = [b for b in body if "semio_s_artifact_stdio_png::register();" in b]
        if not (len(reg) == 1 and len(png) == 1 and len(body) == 5 and body[0].strip() == "static ONCE: std::sync::Once = std::sync::Once::new();" and body[1].strip() == "ONCE.call_once(|| {" and body[4].strip() == "});"):
            raise SystemExit(f"FAIL-CLOSED helper body shape in {full}: {len(body)}")
        regline = reg[0].replace("register(&[", "register(&[", 1).rstrip()
        regline = regline.rstrip(";") + "?;"
        newhelper = [
            "fn ensure_stdio_semio_and_png_registered() -> Result<(), " + RET + "> {",
            "    static RESULT: std::sync::OnceLock<Result<(), " + RET + ">> = std::sync::OnceLock::new();",
            "    RESULT.get_or_init(|| {",
            "        " + regline.strip(),
            "        semio_s_artifact_stdio_png::register()",
            "    }).clone()",
            "}",
        ]
        newlines = lines[:hstart] + newhelper + lines[hend+1:]
        out = "\n".join(newlines)
        callers = [i for i, l in enumerate(newlines) if l.strip() == "ensure_stdio_semio_and_png_registered();"]
        for i in callers:
            newlines[i] = newlines[i].replace("ensure_stdio_semio_and_png_registered();", "ensure_stdio_semio_and_png_registered().map_err(|error| error.to_string())?;")
        edits_out[full] = "\n".join(newlines)
        if len(callers) == 0: raise SystemExit(f"FAIL-CLOSED no callers in {full}")

    # 6. gis test helper: test code, expect is allowed
    rel = "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️relocated-engine/🦀️.rs"
    t = read(rel)
    t = one(t, "        semio_s_artifact_stdio_semio::register(&[semio_s_artifact_stdio_semio_drawing_import::CONVERSION, semio_s_artifact_stdio_semio_drawing_export::CONVERSION]);",
            "        semio_s_artifact_stdio_semio::register(&[semio_s_artifact_stdio_semio_drawing_import::CONVERSION, semio_s_artifact_stdio_semio_drawing_export::CONVERSION]).expect(\"static Stdio registration in test helper\");", rel)
    edits_out[rel] = t

    # 7. test-only callers with bare register();
    for rel, anchor_line in [("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🧪️tests/🔬️io-registry-unit/🦀️.rs", "        register();"),
                             ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🧬️migrations/🧪️tests/🔬️unit/🦀️.rs", "    register();")]:
        t = read(rel)
        t = one(t, "\n" + anchor_line + "\n", "\n" + anchor_line[:-len("register();")] + "register().expect(\"static test registration\");\n", rel)
        edits_out[rel] = t

def main():
    report = {"sig": 0, "expect": 0, "calls": 0}
    base_files = []
    for dp, dn, fn in os.walk(os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio")):
        dn[:] = [d for d in dn if d not in ("node_modules", "target")]
        for f in fn:
            if not f.endswith(".rs"): continue
            full = os.path.join(dp, f); rel = os.path.relpath(full, ROOT)
            if "🧪️tests" in rel and "/🪞️mirror/" not in rel: continue
            src = open(full, encoding="utf-8", errors="replace").read()
            if re.search(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn register\(\)\s*\{', src) or re.search(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?fn register\(\)\s*\{\s*\}', src):
                base_files.append(rel)
    base_files.sort()
    edits = {}
    for rel in base_files:
        new = cm.process(rel, {"defs": 0, "expects": 0, "calls": 0, "manual_defs_missing": [], "manual_calls": []})
        if new is None: raise SystemExit(f"FAIL-CLOSED unexpected None {rel}")
        edits[rel] = new
    special(edits)
    # 8. boundary scan: any remaining register-call or expect in the touched stdio tree that is not converted
    boundary = []
    for dp, dn, fn in os.walk(os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio")):
        dn[:] = [d for d in dn if d not in ("node_modules", "target")]
        for f in fn:
            if not f.endswith(".rs"): continue
            rel = os.path.relpath(os.path.join(dp, f), ROOT)
            if rel in edits: continue
            for k, L in enumerate(open(os.path.join(ROOT, rel), encoding="utf-8", errors="replace"), 1):
                if re.search(r'(?<![A-Za-z0-9_])(?:[A-Za-z_][A-Za-z0-9_]*::)*register\(\)|static Stdio registration', L) and not L.lstrip().startswith("//"):
                    boundary.append(f"{rel}:{k}: {L.strip()[:140]}")
    # write flat store
    os.makedirs(FLAT, exist_ok=True); os.makedirs(PRE, exist_ok=True)
    for f in os.listdir(FLAT): os.remove(os.path.join(FLAT, f))
    for f in os.listdir(PRE): os.remove(os.path.join(PRE, f))
    mapping = {}
    for rel, new in sorted(edits.items()):
        sha = hashlib.sha1(rel.encode("utf-8")).hexdigest()
        pre = open(os.path.join(ROOT, rel), "rb").read()
        post = new.encode("utf-8")
        if post == pre: raise SystemExit(f"FAIL-CLOSED no change {rel}")
        open(os.path.join(FLAT, sha + ".rs"), "wb").write(post)
        open(os.path.join(PRE, sha + ".rs"), "wb").write(pre)
        mapping[sha] = {"repo_path": rel, "pre_sha256": hashlib.sha256(pre).hexdigest(), "post_sha256": hashlib.sha256(post).hexdigest()}
    json.dump(mapping, open(os.path.join(OUT, "mapping.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    json.dump({"files": len(edits), "boundary_remaining": boundary}, open(os.path.join(OUT, "stage1-summary.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(json.dumps({"files_changed": len(edits), "boundary_remaining": len(boundary)}, ensure_ascii=False))
    for b in boundary[:40]: print("BOUNDARY", b[:220])

if __name__ == "__main__":
    main()
