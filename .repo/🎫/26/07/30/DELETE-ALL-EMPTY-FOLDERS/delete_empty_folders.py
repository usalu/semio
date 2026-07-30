import os
import shutil
import subprocess

def main():
    # 1. Run git ls-files to get all tracked files
    res = subprocess.run(
        ['git', 'ls-files'], capture_output=True, text=True, check=True
    )
    tracked_files = [line.strip() for line in res.stdout.splitlines() if line.strip()]

    # 2. Build set of all directories containing tracked files
    dirs_with_tracked = set()
    dirs_with_tracked.add('.')

    for f in tracked_files:
        parts = f.split('/')
        for i in range(1, len(parts)):
            d = '/'.join(parts[:i])
            dirs_with_tracked.add(d)

    # Protect ticket directory and .git
    ticket_dir = '.repo/🎫/26/07/30/DELETE-ALL-EMPTY-FOLDERS'
    ticket_parts = ticket_dir.split('/')
    for i in range(1, len(ticket_parts) + 1):
        dirs_with_tracked.add('/'.join(ticket_parts[:i]))

    dirs_to_delete = []

    for root, dirs, files in os.walk('.', topdown=True):
        norm_root = os.path.normpath(root)

        if norm_root == '.git' or norm_root.startswith('.git/') or norm_root.startswith('.git\\'):
            dirs.clear()
            continue

        if norm_root == '.':
            continue

        if norm_root not in dirs_with_tracked:
            dirs_to_delete.append(norm_root)
            dirs.clear()

    print(f'Found {len(dirs_to_delete)} empty directory trees (no git-tracked files inside).')

    deleted_count = 0
    failed_count = 0

    def handle_remove_readonly(func, path, exc_info):
        try:
            os.chmod(path, 0o777)
            func(path)
        except Exception as e:
            print(f'Failed to remove {path}: {e}')

    for d in dirs_to_delete:
        try:
            if os.path.islink(d):
                os.unlink(d)
            else:
                shutil.rmtree(d, onerror=handle_remove_readonly)
            deleted_count += 1
        except Exception as e:
            print(f'Error deleting {d}: {e}')
            failed_count += 1

    print(f'Successfully deleted {deleted_count} empty directory trees. Failed: {failed_count}')

if __name__ == '__main__':
    main()
