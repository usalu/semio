import re, sys, pathlib


def add(path, names):
    text = pathlib.Path(path).read_text(encoding='utf-8')
    lines = text.split('\n')
    start = next(i for i, l in enumerate(lines) if l.strip() == '[dependencies]')
    end = next((i for i in range(start + 1, len(lines)) if lines[i].startswith('[')), len(lines))
    present = {re.match(r'^([\w-]+)\s*=', l).group(1) for l in lines[start + 1:end] if re.match(r'^([\w-]+)\s*=', l)}
    insert = [f'{n} = {{ workspace = true }}' for n in names if n not in present]
    if not insert:
        return []
    last = max(i for i in range(start + 1, end) if lines[i].strip()) 
    lines[last + 1:last + 1] = insert
    pathlib.Path(path).write_text('\n'.join(lines), encoding='utf-8')
    return insert


if __name__ == '__main__':
    print(add(sys.argv[1], sys.argv[2:]))
