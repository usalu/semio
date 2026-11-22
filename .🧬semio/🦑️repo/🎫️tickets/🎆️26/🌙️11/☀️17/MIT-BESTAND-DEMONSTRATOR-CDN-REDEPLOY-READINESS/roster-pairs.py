#!/usr/bin/env python3
"""🧭️ Schedule the remaining sealed browser roster with two total root slots."""
import argparse
from contextlib import contextmanager
import errno
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
GENERATED = TICKET / '🗑️generated'
MODULES = ROOT / '.cache/b/p/m/browser/release/modules'
CASES = ROOT / '♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json'
PAIRS = [
    ('flow-extension-text', 'flow-extension-math'),
    ('flow', 'playbook'),
    ('process-extension-metal', 'process-extension-wood'),
    ('sourcing-module-beams', 'sourcing-module-slabs'),
    ('cad-extension-aec-building-energy', 'cad-extension-aec-building-structure'),
    ('flow-extension-draw', 'flow-extension-list'),
    ('flow-extension-logic', 'flow-extension-primitive'),
    ('process-extension-concrete', 'process-extension-robotic'),
    ('flow-extension-brep', 'playbook-module-procedural'),
    ('cad-extension-aec-building', 'cad-extension-spatial-shape'),
]
SLOTS = int(os.environ.get('ROSTER_SLOTS', '2'))
PRIORITY = ['demonstrator', 'procedural', 'gis', 'cad']


def windows_process(pid):
    if not isinstance(pid, int) or pid < 1:
        raise ValueError('invalid process id')
    script = ("$ErrorActionPreference='Stop'; "
              f"$p=Get-CimInstance -ClassName Win32_Process -Filter 'ProcessId = {pid}'; "
              "if ($null -eq $p) { 'null' } else { "
              "@{identity=$p.CreationDate.ToUniversalTime().ToString('o');command=$p.CommandLine} | ConvertTo-Json -Compress }")
    result = subprocess.run(['powershell.exe', '-NoProfile', '-NonInteractive', '-Command', script], text=True, capture_output=True)
    if result.returncode != 0:
        raise RuntimeError(f'Windows process query failed for {pid}: {result.stderr.strip()}')
    try:
        row = json.loads(result.stdout.lstrip('\ufeff').strip())
    except json.JSONDecodeError as error:
        raise RuntimeError(f'invalid Windows process query for {pid}') from error
    if row is None:
        return '', ''
    if not isinstance(row, dict) or not isinstance(row.get('identity'), str) or not row['identity'] or not isinstance(row.get('command'), str):
        raise RuntimeError(f'incomplete Windows process query for {pid}')
    return row['identity'], row['command']


def process_identity(pid):
    if os.name == 'nt':
        return windows_process(pid)[0]
    result = subprocess.run(['ps', '-p', str(pid), '-o', 'stat=', '-o', 'lstart=', '-o', 'command='], text=True, capture_output=True)
    value = result.stdout.strip() if result.returncode == 0 else ''
    fields = value.split(None, 6)
    return '' if len(fields) < 7 or fields[0].startswith('Z') else ' '.join(fields[1:6])


def process_command(pid):
    if os.name == 'nt':
        return windows_process(pid)[1]
    result = subprocess.run(['ps', '-p', str(pid), '-o', 'command='], text=True, capture_output=True)
    return result.stdout.strip() if result.returncode == 0 else ''


def queue_command_matches(command, roots):
    shell = r'(?:"[^"\r\n]*[/\\](?:bash|sh)(?:\.exe)?"|(?:[^\s"\']*[/\\])?(?:bash|sh)(?:\.exe)?)'
    path = r'(?:"[^"\r\n]*[/\\]native-queue\.sh"|\'[^\'\r\n]*[/\\]native-queue\.sh\'|[^\s"\']*[/\\]native-queue\.sh|native-queue\.sh)'
    arguments = (r'--pairs\s+' if len(roots) == 2 else '') + r'\s+'.join(re.escape(root) for root in roots)
    return re.match(r'^' + shell + r'\s+' + path + r'\s+' + arguments + r'(?:\s|$)', command, re.IGNORECASE) is not None


def receipt_valid(module, modules=MODULES, confirmed=GENERATED / 'confirmed'):
    spec = importlib.util.spec_from_file_location('release_receipts', TICKET / 'release-receipts.py')
    helper = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helper)
    try:
        return helper.read_receipt(confirmed / f'{module}.json', module) == helper.read_receipt(helper.receipt_path(modules, module), module)
    except (OSError, ValueError, KeyError, TypeError):
        return False


def receipt_stamp(module, confirmed=GENERATED / 'confirmed'):
    try:
        row = (confirmed / f'{module}.json').stat()
        return [row.st_ino, row.st_mtime_ns, row.st_size]
    except FileNotFoundError:
        return None


def save(path, state):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + f'.{os.getpid()}.pending')
    temporary.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')
    temporary.replace(path)


def request_directory(state_path):
    return state_path.with_name(state_path.name + '.requests')


def definitely_dead(pid):
    if os.name == 'nt':
        return windows_process(pid) == ('', '')
    try:
        os.kill(pid, 0)
    except OSError as error:
        if error.errno == errno.ESRCH:
            return True
        raise RuntimeError(f'cannot establish process status: {pid}') from error
    return False


@contextmanager
def coordinator_lock(state_path):
    marker = state_path.with_name(state_path.name + '.lock')
    sidecar = state_path.with_name(state_path.name + '.lock.advisory')
    with sidecar.open('a+b') as handle:
        if os.name == 'nt':
            import msvcrt
            handle.seek(0, os.SEEK_END)
            if handle.tell() == 0:
                handle.write(b'\0'); handle.flush()
            handle.seek(0)
            try:
                msvcrt.locking(handle.fileno(), msvcrt.LK_NBLCK, 1)
            except OSError as error:
                raise RuntimeError('roster coordinator advisory lock held') from error
        else:
            import fcntl
            try:
                fcntl.flock(handle.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
            except BlockingIOError as error:
                raise RuntimeError('roster coordinator advisory lock held') from error
        try:
            if marker.exists():
                try:
                    if not marker.is_dir() or marker.is_symlink() or {p.name for p in marker.iterdir()} != {'owner.json'}:
                        raise ValueError('unexpected marker contents')
                    owner = json.loads((marker / 'owner.json').read_text())
                    pid = owner['pid']
                    if type(pid) is not int or pid < 1 or not isinstance(owner.get('identity'), str) or not owner['identity'] or owner.get('state') != str(state_path.resolve()) or owner.get('retry_protocol') != 1:
                        raise ValueError('invalid owner')
                    if not definitely_dead(pid):
                        raise RuntimeError(f'roster owner is live or ambiguous: {pid}')
                except (OSError, KeyError, TypeError, ValueError) as error:
                    raise RuntimeError(f'unverifiable roster owner: {marker}') from error
            else:
                marker.mkdir()
            identity = process_identity(os.getpid())
            if not identity:
                raise RuntimeError('cannot establish coordinator identity')
            save(marker / 'owner.json', {'pid': os.getpid(), 'identity': identity, 'state': str(state_path.resolve()), 'retry_protocol': 1})
            try:
                yield
            finally:
                (marker / 'owner.json').unlink()
                marker.rmdir()
        finally:
            if os.name == 'nt':
                handle.seek(0); msvcrt.locking(handle.fileno(), msvcrt.LK_UNLCK, 1)
            else:
                fcntl.flock(handle.fileno(), fcntl.LOCK_UN)


def request_retry(state_path, module, roster, identity=process_identity, command=process_command):
    if module not in roster:
        raise ValueError(f'unknown roster module: {module}')
    lock = state_path.with_name(state_path.name + '.lock')
    try:
        owner = json.loads((lock / 'owner.json').read_text())
        pid = owner['pid']
        if owner.get('retry_protocol') != 1 or not isinstance(pid, int) or owner.get('state') != str(state_path.resolve()):
            raise RuntimeError('active coordinator does not support retry requests')
        if not owner.get('identity') or identity(pid) != owner.get('identity') or not re.search(r'(?:^|[ /\\])roster-pairs\.py(?:["\']?\s|["\']?$)', command(pid)):
            raise RuntimeError('coordinator identity or command changed')
    except (FileNotFoundError, KeyError, json.JSONDecodeError) as error:
        raise RuntimeError('no verifiable active coordinator') from error
    requests = request_directory(state_path)
    requests.mkdir(parents=True, exist_ok=True)
    try:
        with (requests / module).open('x'):
            pass
        return True
    except FileExistsError:
        return False


def run_scheduler(roster, state_path, predecessors, launch, valid, identity=process_identity, sleep=time.sleep, interval=5, command=process_command, stamp=receipt_stamp):
    if state_path.exists():
        state = json.loads(state_path.read_text())
        if state.get('version') != 1:
            raise ValueError('unsupported roster state')
    else:
        state = {'version': 1, 'forced_demonstrator': False, 'failed': [], 'active': []}
    retry_due = set(state.get('retry_due', [])) | set(state.get('failed', []))
    if not retry_due.issubset(roster):
        raise ValueError('saved retry roots are not in the current roster')
    failed = set()
    active = state['active']
    for record in active:
        current = identity(record['pid'])
        if current and (current != record['identity'] or not queue_command_matches(command(record['pid']), record['roots'])):
            raise RuntimeError(f"PID identity or command changed: {record['pid']}")
    for module, pid in predecessors.items():
        if any(module in row['roots'] for row in active):
            continue
        current = identity(pid)
        if not current:
            if not valid(module):
                failed.add(module)
            continue
        if not queue_command_matches(command(pid), [module]):
            raise RuntimeError(f'predecessor identity does not match: {module}={pid}')
        active.append({'roots': [module], 'pid': pid, 'identity': current, 'predecessor': True, 'before': {module: stamp(module)}})
    if sum(len(x['roots']) for x in active) > SLOTS:
        raise RuntimeError('more than two root slots already active')
    state['failed'] = sorted(failed)
    state['retry_due'] = sorted(retry_due)
    save(state_path, state)
    while True:
        for record in active[:]:
            process = record.get('_process')
            current = '' if process is not None and process.poll() is not None else identity(record['pid'])
            if current:
                if current != record['identity'] or (process is None and not queue_command_matches(command(record['pid']), record['roots'])):
                    raise RuntimeError(f"PID identity or command changed: {record['pid']}")
                continue
            if process is not None:
                process.wait()
            for module in record['roots']:
                if valid(module) and stamp(module) != record['before'].get(module):
                    retry_due.discard(module)
                    if module == 'demonstrator' and not record.get('predecessor'):
                        state['forced_demonstrator'] = True
                else:
                    failed.add(module)
            active.remove(record)
        busy = {module for row in active for module in row['roots']}
        slots = SLOTS - sum(len(row['roots']) for row in active)
        requests = request_directory(state_path)
        for module in roster:
            path = requests / module
            if module not in busy and path.exists():
                retry_due.add(module)
                failed.discard(module)
                state['retry_due'] = sorted(retry_due)
                state['failed'] = sorted(failed)
                state['active'] = [{key: value for key, value in row.items() if key != '_process'} for row in active]
                save(state_path, state)
                state['active'] = active
                path.unlink(missing_ok=True)
        def needed(module):
            return module not in busy and module not in failed and (module in retry_due or module == 'demonstrator' and not state['forced_demonstrator'] or not valid(module))
        pending = [module for module in roster if needed(module)]
        for module in PRIORITY:
            if not slots:
                break
            if module not in pending or (module == 'cad' and 'procedural' in busy) or (module == 'procedural' and 'cad' in busy):
                continue
            before = {module: stamp(module)}
            process, fingerprint = launch([module])
            active.append({'roots': [module], 'pid': process.pid, 'identity': fingerprint, 'predecessor': False, 'before': before, '_process': process})
            busy.add(module); slots -= 1
        complete_pairs = [list(pair) for pair in PAIRS if all(module in roster and needed(module) for module in pair)]
        if slots >= 2 and complete_pairs:
            pair = complete_pairs[0]
            before = {module: stamp(module) for module in pair}
            process, fingerprint = launch(pair)
            active.append({'roots': pair, 'pid': process.pid, 'identity': fingerprint, 'predecessor': False, 'before': before, '_process': process})
            busy.update(pair); slots = 0
        paired_pending = {module for pair in complete_pairs for module in pair}
        singles = [module for module in roster if needed(module) and module not in PRIORITY and module not in paired_pending]
        while slots and singles:
            module = singles.pop(0)
            before = {module: stamp(module)}
            process, fingerprint = launch([module])
            active.append({'roots': [module], 'pid': process.pid, 'identity': fingerprint, 'predecessor': False, 'before': before, '_process': process})
            busy.add(module); slots -= 1
        state['failed'] = sorted(failed)
        state['retry_due'] = sorted(retry_due)
        state['active'] = [{key: value for key, value in row.items() if key != '_process'} for row in active]
        save(state_path, state)
        state['active'] = active
        if not active:
            return not failed
        sleep(interval)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--predecessor', action='append', default=[], metavar='MODULE=PID')
    parser.add_argument('--state', type=Path, default=GENERATED / 'roster-pairs-state.json')
    parser.add_argument('--retry', metavar='MODULE')
    args = parser.parse_args()
    predecessors = {}
    for item in args.predecessor:
        module, separator, pid = item.partition('=')
        if not separator or not re.fullmatch(r'[a-z0-9]+(?:-[a-z0-9]+)*', module) or not pid.isdecimal() or int(pid) < 1 or module in predecessors:
            parser.error('predecessor must be a unique MODULE=PID')
        predecessors[module] = int(pid)
    data = json.loads(CASES.read_text())
    roster = list(dict.fromkeys(data['pluginIds'] + data['extensionIds']))
    if args.retry:
        if args.predecessor:
            parser.error('--retry cannot be combined with --predecessor')
        created = request_retry(args.state, args.retry, roster)
        print('retry request queued' if created else 'retry request already queued', args.retry)
        return 0
    def launch(roots):
        command = ['bash', str(TICKET / 'native-queue.sh')] + (['--pairs'] if len(roots) == 2 else []) + roots
        env = os.environ.copy(); env['JOBS'] = '2'
        process = subprocess.Popen(command, env=env)
        fingerprint = process_identity(process.pid)
        if not fingerprint:
            process.wait(); raise RuntimeError(f'queue exited before identity capture: {roots}')
        print('[roster start]', *roots, 'pid', process.pid, flush=True)
        return process, fingerprint
    with coordinator_lock(args.state):
        return 0 if run_scheduler(roster, args.state, predecessors, launch, receipt_valid) else 1


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError, RuntimeError) as error:
        sys.exit(str(error))
