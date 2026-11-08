import re, sys, os
A = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
M = A + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
REL = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
PLAN = {"🪛fastened-joints": ("fastened-joints", 3), "🪜buckling-lengths": ("buckling-lengths", 2), "⏳fatigue-stresses": ("fatigue-stresses", 2)}
def run(d, slug, parts):
    lib = open(f"{M}/{d}/🦀️.rs").read()
    mounts = re.findall(r'(#\[path = "[^"]+"\]\npub mod (\w+);\n)', lib)
    head, rest = lib.split("//#region 🪢️Mounts\n")
    tail = rest.split("//#endregion 🪢️Mounts")[1]
    n = len(mounts); size = -(-n // parts)
    chunks = [mounts[i:i+size] for i in range(0, n, size)]
    cargo = open(f"{M}/{d}/📦️packages/🦀️rust/Cargo.toml").read()
    root = open(f"{A}/Cargo.toml").read()
    uses = ""
    for i, ch in enumerate(chunks[1:], 2):
        nd = f"{d}-{i}"; nslug = f"{slug}-{i}"
        os.makedirs(f"{M}/{nd}/📦️packages/🦀️rust", exist_ok=True)
        h = head.replace("//! ", f"//! (part {i}) ", 1)
        open(f"{M}/{nd}/🦀️.rs", "w").write(h + "//#region 🪢️Mounts\n" + "".join(m[0] for m in ch) + "//#endregion 🪢️Mounts" + tail)
        c = cargo.replace(f"mut-{slug}", f"mut-{nslug}")
        open(f"{M}/{nd}/📦️packages/🦀️rust/Cargo.toml", "w").write(c)
        crate = f"semio_s_artifact_norm_en1993_mut_{nslug.replace('-', '_')}"
        uses += f"pub use {crate}::{{" + ", ".join(m[1] for m in ch) + "};\n"
        root = root.replace(f'    "{REL}/{d}/📦️packages/🦀️rust",\n', f'    "{REL}/{d}/📦️packages/🦀️rust",\n    "{REL}/{nd}/📦️packages/🦀️rust",\n', 1)
        root = root.replace(f'[workspace.dependencies.semio-s-artifact-norm-en1993-mut-{slug}]\npath = "{REL}/{d}/📦️packages/🦀️rust"\n', f'[workspace.dependencies.semio-s-artifact-norm-en1993-mut-{slug}]\npath = "{REL}/{d}/📦️packages/🦀️rust"\n\n[workspace.dependencies.semio-s-artifact-norm-en1993-mut-{nslug}]\npath = "{REL}/{nd}/📦️packages/🦀️rust"\n', 1)
        oc = cargo
        cargo_dep = f"semio-s-artifact-norm-en1993-mut-{nslug} = {{ workspace = true }}\n"
        oc = oc.replace("[features]", cargo_dep + "\n[features]", 1) if cargo_dep not in oc else oc
        cargo = oc
        script = f"{A}/📦️packages/🦀️rust/📜️script.ts"
        s = open(script).read()
        s = s.replace(f'"semio-s-artifact-norm-en1993-mut-{slug}"', f'"semio-s-artifact-norm-en1993-mut-{slug}","semio-s-artifact-norm-en1993-mut-{nslug}"', 1)
        open(script, "w").write(s)
    open(f"{M}/{d}/📦️packages/🦀️rust/Cargo.toml", "w").write(cargo)
    open(f"{A}/Cargo.toml", "w").write(root)
    open(f"{M}/{d}/🦀️.rs", "w").write(head + "//#region 🪢️Mounts\n" + "".join(m[0] for m in chunks[0]) + "//#endregion 🪢️Mounts" + tail.rstrip("\n") + "\n\n" + uses)
for d, (slug, p) in PLAN.items(): run(d, slug, p)
