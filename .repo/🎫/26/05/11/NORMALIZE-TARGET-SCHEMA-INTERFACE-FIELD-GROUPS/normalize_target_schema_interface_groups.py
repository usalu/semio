from __future__ import annotations
import difflib
import re
from pathlib import Path

path = Path(r"c:/git/semio/semio/graphql/target.schema.graphql")
text = path.read_text(encoding="utf-8")
lines = text.splitlines()

block_open = re.compile(r'^(interface|type)\s+(\w+)\s+implements\s+([^\{]+)\{$')
field_re = re.compile(r'^  [A-Za-z_][A-Za-z0-9_]*(?:\([^)]*\))?: ')


def is_field(line: str) -> bool:
    return bool(field_re.match(line))


def is_base_entity_shell(chunk: list[str], marker: str) -> bool:
    return len(chunk) >= 5 and chunk[0] == f'  # {marker}' and chunk[1].startswith('  id: ID!') and chunk[2].startswith('  hash: String!') and chunk[3].startswith('  owner: Entity') and chunk[4].startswith('  owns: EntityConnection')


def split_base_entity(chunk: list[str]) -> list[str]:
    return ['  # Node', chunk[1], '  # Entity', chunk[2], chunk[3], chunk[4]] + chunk[5:]


def maybe_split_artifact(chunk: list[str]) -> list[str]:
    if len(chunk) < 13:
        return chunk
    checks = [
        chunk[0] == '  # Artifact',
        chunk[1].startswith('  name: String!'),
        chunk[2].startswith('  description: String!'),
        chunk[3].startswith('  icon: String!'),
        chunk[4].startswith('  createdAt: Timestamp'),
        chunk[5].startswith('  createdBy: Author'),
        chunk[6].startswith('  authoredBy: AuthorConnection'),
        chunk[7].startswith('  changedIn: CheckpointConnection'),
        chunk[8].startswith('  lastChangedAt: Timestamp'),
        chunk[9].startswith('  lastChangedBy: Author'),
        chunk[10].startswith('  lastChangedIn: Checkpoint'),
        chunk[11].startswith('  changes: ChangeConnection'),
        chunk[12].startswith('  edits: EditConnection'),
    ]
    if not all(checks):
        return chunk
    return [
        '  # RichStrongEntity',
        chunk[1],
        chunk[2],
        chunk[3],
        chunk[4],
        chunk[5],
        '  # Artifact',
        chunk[6],
        chunk[7],
        chunk[8],
        chunk[9],
        chunk[10],
        chunk[11],
        chunk[12],
        *chunk[13:],
    ]


def normalize_block(header: str, body: list[str]) -> list[str]:
    out = body[:]
    if len(out) >= 6 and out[0] == '  # Node' and out[1].startswith('  id: ID!') and out[2] == '  # StrongEntity' and out[3].startswith('  hash: String!') and out[4].startswith('  owner: Entity') and out[5].startswith('  owns: EntityConnection'):
        out = ['  # Node', out[1], '  # Entity', out[3], out[4], out[5], *out[6:]]
    for marker in ('WeakEntity', 'StrongEntity', 'Diff', 'Modification'):
        if is_base_entity_shell(out, marker):
            out = split_base_entity(out)
            break
    if len(out) >= 6 and out[0] == '  # Node' and out[1].startswith('  id: ID!') and out[2] == '  # Entity' and out[3].startswith('  hash: String!') and out[4].startswith('  owner: Entity') and out[5].startswith('  owns: EntityConnection'):
        out = out[:6] + maybe_split_artifact(out[6:])
    if len(out) >= 6 and out[0] == '  # Node' and out[1].startswith('  id: ID!') and out[2] == '  # Entity' and out[3].startswith('  hash: String!') and out[4].startswith('  owner: Entity') and out[5].startswith('  owns: EntityConnection') and len(out) > 6 and is_field(out[6]):
        name = block_open.match(header).group(2)
        out = out[:6] + [f'  # {name}'] + out[6:]
    return out

new_lines: list[str] = []
i = 0
while i < len(lines):
    line = lines[i]
    match = block_open.match(line)
    if not match:
        new_lines.append(line)
        i += 1
        continue
    block = [line]
    i += 1
    while i < len(lines):
        block.append(lines[i])
        if lines[i] == '}':
            i += 1
            break
        i += 1
    body = block[1:-1]
    normalized = normalize_block(block[0], body)
    new_lines.extend([block[0], *normalized, '}'])

if new_lines == lines:
    print('No changes needed')
    raise SystemExit(0)

path.write_text('\n'.join(new_lines) + '\n', encoding='utf-8')

old = [line + '\n' for line in lines]
new = [line + '\n' for line in new_lines]
matcher = difflib.SequenceMatcher(a=lines, b=new_lines)
patch_lines = ['*** Begin Patch', f'*** Update File: {path.as_posix()}']
context = 3
for tag, i1, i2, j1, j2 in matcher.get_opcodes():
    if tag == 'equal':
        continue
    pre = old[max(0, i1 - context):i1]
    post = old[i2:min(len(old), i2 + context)]
    patch_lines.extend(line.rstrip('\n') for line in pre)
    patch_lines.extend('-' + line.rstrip('\n') for line in old[i1:i2])
    patch_lines.extend('+' + line.rstrip('\n') for line in new[j1:j2])
    patch_lines.extend(line.rstrip('\n') for line in post)
    patch_lines.append('')
patch_lines.append('*** End Patch')
patch_path = Path(r"c:/git/semio/.repo/🎫/26/05/11/NORMALIZE-TARGET-SCHEMA-INTERFACE-FIELD-GROUPS/target-schema-interface-groups.patch")
patch_path.write_text('\n'.join(patch_lines), encoding='utf-8')
print(f'Wrote patch to {patch_path}')
print(f'Old lines: {len(lines)} New lines: {len(new_lines)}')
print(f'Opcode count: {len([op for op in matcher.get_opcodes() if op[0] != "equal"])}')
