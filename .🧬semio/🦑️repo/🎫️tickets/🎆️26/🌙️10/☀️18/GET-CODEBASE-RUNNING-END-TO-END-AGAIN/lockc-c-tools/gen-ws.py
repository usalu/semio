import os, random, sys, shutil
root = sys.argv[1]
seed = int(sys.argv[2]) if len(sys.argv) > 2 else 7
random.seed(seed)
shutil.rmtree(root, ignore_errors=True)
layers = [["l0"], [f"a{i}" for i in range(6)], [f"b{i}" for i in range(12)], [f"c{i}" for i in range(12)], [f"t{i}" for i in range(8)]]
os.makedirs(root)
members = [n for l in layers for n in l]
open(os.path.join(root, "Cargo.toml"), "w").write('[workspace]\nresolver = "3"\nmembers = [%s]\n[profile.dev]\ndebug = 0\nincremental = false\n' % ", ".join('"%s"' % m for m in members))
os.makedirs(os.path.join(root, ".cargo"))
open(os.path.join(root, ".cargo", "config.toml"), "w").write('[unstable]\nembed-metadata = "DoNotEmbed"\nfine-grain-locking = true\nbuild-dir-new-layout = true\nchecksum-freshness = true\n')
for li, layer in enumerate(layers):
    for n in layer:
        d = os.path.join(root, n); os.makedirs(os.path.join(d, "src"))
        deps = []
        if li > 0:
            prev = layers[li - 1] + (layers[li - 2] if li > 1 else [])
            deps = random.sample(prev, min(len(prev), random.randint(2, 4)))
        toml = '[package]\nname = "%s"\nversion = "0.1.0"\nedition = "2021"\nbuild = "build.rs"\n[features]\nf1 = []\nf2 = []\n[dependencies]\n' % n
        for dd in deps: toml += '%s = { path = "../%s" }\n' % (dd, dd)
        open(os.path.join(d, "Cargo.toml"), "w").write(toml)
        open(os.path.join(d, "build.rs"), "w").write('fn main(){ println!("cargo:rerun-if-env-changed=LOCKC_VARIANT"); println!("cargo:rerun-if-changed=src/lib.rs"); std::thread::sleep(std::time::Duration::from_millis(150)); }\n')
        body = "".join("pub use %s as dep_%s;\n" % (dd, dd) for dd in deps)
        body += "".join("pub fn f%d(x: u64) -> u64 { x.wrapping_mul(%d) ^ %d }\n" % (k, k + 3, k * 7) for k in range(40))
        open(os.path.join(d, "src", "lib.rs"), "w").write(body)
print("members", len(members))
