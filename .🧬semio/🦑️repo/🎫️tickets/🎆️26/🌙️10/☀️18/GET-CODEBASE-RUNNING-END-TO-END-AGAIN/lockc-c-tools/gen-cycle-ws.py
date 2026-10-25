import os, shutil, sys
root = sys.argv[1]
shutil.rmtree(root, ignore_errors=True)
os.makedirs(os.path.join(root, ".cargo"))
open(os.path.join(root, "Cargo.toml"), "w").write('[workspace]\nresolver = "3"\nmembers = ["x", "y", "t"]\n[profile.dev]\ndebug = 0\nincremental = false\n')
open(os.path.join(root, ".cargo", "config.toml"), "w").write('[unstable]\nembed-metadata = "DoNotEmbed"\nfine-grain-locking = true\nbuild-dir-new-layout = true\nchecksum-freshness = true\n')
def crate(name, var, deps):
    d = os.path.join(root, name); os.makedirs(os.path.join(d, "src"))
    t = '[package]\nname = "%s"\nversion = "0.1.0"\nedition = "2021"\nbuild = "build.rs"\n[dependencies]\n' % name
    for dd in deps: t += '%s = { path = "../%s" }\n' % (dd, dd)
    open(os.path.join(d, "Cargo.toml"), "w").write(t)
    open(os.path.join(d, "build.rs"), "w").write("fn main(){ let _ = option_env!(\"%s\"); }\n" % var)
    open(os.path.join(d, "src", "lib.rs"), "w").write("pub fn f() -> u32 { 1 }\n")
crate("x", "XVAR", []); crate("y", "YVAR", []); crate("t", "TVAR", ["x", "y"])
