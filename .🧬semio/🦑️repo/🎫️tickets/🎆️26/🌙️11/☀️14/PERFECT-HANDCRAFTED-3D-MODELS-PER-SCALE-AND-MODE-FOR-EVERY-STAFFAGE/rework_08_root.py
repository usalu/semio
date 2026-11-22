"""🪡 Splices changed root members (rig, poses, clips, anchors, representations, thermal, structure, dimensions) of an asset into its catalog file or a candidate copy; untouched members stay byte-identical (rework author 08)."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rework_08_fk import path

T = os.path.dirname(os.path.abspath(__file__))


def member_end(text, start):
    """🔚 Index after the JSON value that starts at start."""
    depth, index, quoted = 0, start, False
    if text[start] not in '{[':
        return re.compile(r'[,\n]').search(text, start).start()
    while True:
        char = text[index]
        if quoted:
            if char == '\\':
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in '{[':
            depth += 1
        elif char in '}]':
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1


def render(value):
    """🖨️ Member text at asset depth with two-space indentation."""
    return json.dumps(value, ensure_ascii=False, indent=2).replace('\n', '\n    ')


def scalar(value):
    """🔹 True for a JSON leaf."""
    return not isinstance(value, (dict, list))


def inline(value):
    """➖ True for a list the catalog keeps on one line."""
    return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)


def compact(value, depth=2):
    """🗜️ Member text in the catalog's compact style (scalar lists and short point lists inline)."""
    pad, inner = '  ' * depth, '  ' * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return '{}'
        return '{\n' + ',\n'.join(inner + json.dumps(key, ensure_ascii=False) + ': ' + compact(item, depth + 1) for key, item in value.items()) + '\n' + pad + '}'
    if isinstance(value, list):
        if not value:
            return '[]'
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return '[\n' + ',\n'.join(inner + compact(item, depth + 1) for item in value) + '\n' + pad + ']'
    return json.dumps(value, ensure_ascii=False)


def splice(slug, change, candidate=False):
    """✍️ Re-reads the catalog file, lets change(asset) return the set of changed member keys, and writes the candidate (default) or the catalog file."""
    source = path(slug)
    text = open(source, encoding='utf-8').read()
    document = json.loads(text)
    before = json.loads(text)['asset']
    keys = change(document['asset'])
    result = text
    for key in keys:
        found = re.search(r'\n    "' + key + r'"\s*:\s*', result)
        end = member_end(result, found.end())
        style = render if result[found.end():end] == render(before[key]) else compact
        result = result[:found.end()] + style(document['asset'][key]) + result[end:]
    if json.loads(result) != document:
        raise SystemExit('splice did not round-trip')
    if candidate:
        folder = f'{T}/🗑️generated/r3-{slug}'
        os.makedirs(folder, exist_ok=True)
        target = f'{folder}/🔣️.json'
    else:
        if open(source, encoding='utf-8').read() != text:
            raise SystemExit('asset changed while splicing; retry')
        target = source
    open(target, 'w', encoding='utf-8').write(result)
    print('wrote', target, 'members', sorted(keys))
    return target
