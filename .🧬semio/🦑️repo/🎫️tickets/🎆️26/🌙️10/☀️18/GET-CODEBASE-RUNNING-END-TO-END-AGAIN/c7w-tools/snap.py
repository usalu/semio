"""📸 c7w snapshot: snap.py <tag> <repo-relative path> copies the live file to orig/<tag>.rs (refuses to overwrite) and records the sha."""
import sys, os, shutil, hashlib
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = '/Users/ueli/Documents/semio'
tag, rel = sys.argv[1], sys.argv[2]
dst = f'{HERE}/orig/{tag}.rs'
src = os.path.join(REPO, rel)
if os.path.exists(dst):
    if open(dst, 'rb').read() != open(src, 'rb').read():
        print('PROBLEM snapshot differs from live', tag); sys.exit(1)
    print('snapshot exists and equals live', tag); sys.exit(0)
shutil.copyfile(src, dst)
print('snapshotted', tag, hashlib.sha256(open(dst, 'rb').read()).hexdigest()[:12], open(dst, 'rb').read().count(b'\n'))
