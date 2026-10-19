"""cstep scanner: syntax-parse Rust files with the stable rustfmt toolchain binary (no cargo). usage: parsecheck.py listfile"""
import subprocess,sys,os
RF=os.path.expanduser('~/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rustfmt')
bad=0;n=0
for p in open(sys.argv[1]).read().split('\n'):
    if not p: continue
    n+=1
    r=subprocess.run([RF,'--edition','2021','--config','skip_children=true','--emit','stdout',p],capture_output=True,text=True)
    if r.returncode!=0:
        bad+=1;print('PARSE-FAIL',p);print('   ',r.stderr.strip().splitlines()[:3])
print('checked',n,'failed',bad,file=sys.stderr)
