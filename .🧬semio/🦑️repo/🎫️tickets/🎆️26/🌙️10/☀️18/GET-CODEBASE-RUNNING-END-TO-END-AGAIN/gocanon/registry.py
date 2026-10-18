#!/usr/bin/env python3
"""registry.py <additions.json>...  anchored insertion into semanticDirectoryMemberKinds["members-of-members-of-members-of-modules"]."""
import json, sys

TAX = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json'
KIND = '    "members-of-members-of-members-of-modules": {\n      "ownerKindIds": ['


def main(files):
    names = []
    for f in files:
        names.extend(json.load(open(f)))
    s = open(TAX, encoding='utf8').read()
    start = s.index(KIND)
    assert s.count(KIND) == 1
    owners_end = s.index('\n      ],', start)
    owners = s[start:owners_end]
    if '"go-language"' not in owners:
        s = s[:owners_end] + ',\n        "go-language"' + s[owners_end:]
    start = s.index(KIND)
    names_open = s.index('"memberNames": [', start)
    names_close = s.index('\n      ]', names_open)
    block = json.loads('[' + s[names_open + len('"memberNames": ['):names_close] + ']')
    fresh = [n for n in dict.fromkeys(names) if n not in block]
    if fresh:
        insert = ',\n'.join('        ' + json.dumps(n, ensure_ascii=False) for n in fresh)
        s = s[:names_close] + ',\n' + insert + s[names_close:]
    json.loads(s)
    open(TAX, 'w', encoding='utf8').write(s)
    print(f'added {len(fresh)} names; owner go-language present')


if __name__ == '__main__':
    main(sys.argv[1:])
