"""🪧 Streams print test.log files and prints the action-time marks that bound the boot: last action heartbeat before each mark, the artifact-rust first tick, the first selection row, the first source-admission row, the first Compiling line, Finished, nextest Summary."""
import re, sys

beat = re.compile(r'^\[@semio-tech/[^\]]+:test:[a-z]+\] running elapsedMs=(\d+)')
art = re.compile(r'^\[artifact-rust:[^\]]+\] running elapsedMs=(\d+)')
fin = re.compile(r'Finished .* in ([0-9.]+)(m?)s?')
for path in sys.argv[1:]:
    t = 0; marks = []; seen = set(); arts = []
    def mark(name, extra=''):
        if name not in seen: seen.add(name); marks.append(f'{name}@>{t}s{extra}')
    for line in open(path, errors='replace'):
        m = beat.match(line)
        if m: t = int(m.group(1)) // 1000; continue
        m = art.match(line)
        if m: arts.append(f'{int(m.group(1))/1000:.1f}(>{t})'); continue
        if '"command":"test","stage":"running"' in line: mark('test-run')
        elif 'task-admission' in line: mark('task-admission')
        elif 'cargo:workspace-selection' in line: mark('selection')
        elif 'cargo:unit-graph' in line: mark('unit-graph')
        elif 'cargo:foreign-source' in line: mark('foreign')
        elif 'cargo:source-admission' in line: mark('admission')
        elif line.lstrip().startswith('Compiling '): mark('compiling')
        elif 'Finished `' in line:
            f = fin.search(line); mark('finished', ' cargo=' + (f.group(1) + f.group(2) if f else '?'))
        elif 'Summary [' in line: mark('summary')
    print(path.rsplit('/', 1)[-1], 'end>', t, '| art ticks', ' '.join(arts[:3]), '|', ' '.join(marks))
