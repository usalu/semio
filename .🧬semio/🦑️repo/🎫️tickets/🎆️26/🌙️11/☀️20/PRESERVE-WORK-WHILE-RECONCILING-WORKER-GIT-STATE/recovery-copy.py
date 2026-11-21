"""Resume a private APFS worktree copy without crossing mounted filesystems."""
import collections
import ctypes
import os
from pathlib import Path
import shutil
import stat
import sys

source, recovery = map(Path, sys.argv[1:])
copy = recovery / 'worktree'
newer = recovery / 'later'
clonefile = ctypes.CDLL(None, use_errno=True).clonefile
volume = source.stat().st_dev
counts = collections.Counter()


def signature(info):
    return info.st_mode, info.st_size, info.st_mtime_ns


def preserve(path, destination, info):
    destination.parent.mkdir(parents=True, exist_ok=True)
    if stat.S_ISLNK(info.st_mode):
        destination.symlink_to(os.readlink(path))
    elif stat.S_ISREG(info.st_mode):
        if clonefile(os.fsencode(path), os.fsencode(destination), 0) != 0:
            error = ctypes.get_errno()
            if error not in (18, 45):
                raise OSError(error, os.strerror(error), str(path))
            shutil.copy2(path, destination, follow_symlinks=False)
    else:
        counts['special'] += 1
        print('special', path.relative_to(source))
        return
    counts['copied'] += 1
    if signature(path.lstat()) != signature(info):
        counts['changed_during_copy'] += 1
        print('changed_during_copy', path.relative_to(source))


def walk(directory):
    for item in os.scandir(directory):
        path = Path(item.path)
        relative = path.relative_to(source)
        if relative.parts[0] in ('.git', '.cache'):
            continue
        try:
            info = item.stat(follow_symlinks=False)
            if info.st_dev != volume:
                counts['mounted'] += 1
                print('mounted', relative)
                continue
            destination = copy / relative
            if stat.S_ISDIR(info.st_mode):
                destination.mkdir(exist_ok=True)
                walk(path)
                continue
            if os.path.lexists(destination):
                same = signature(destination.lstat()) == signature(info)
                if same and stat.S_ISLNK(info.st_mode):
                    same = os.readlink(destination) == os.readlink(path)
                if same:
                    counts['already_copied'] += 1
                    continue
                destination = newer / relative
            preserve(path, destination, info)
        except OSError as error:
            counts['errors'] += 1
            print('error', relative, error)


walk(source)
print(dict(counts))
if counts['errors'] or counts['changed_during_copy']:
    sys.exit(1)
