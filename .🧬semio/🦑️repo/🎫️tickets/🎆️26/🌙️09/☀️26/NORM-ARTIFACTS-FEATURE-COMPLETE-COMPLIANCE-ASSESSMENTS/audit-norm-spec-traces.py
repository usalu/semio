from collections import Counter
import json
from pathlib import Path


def read_json(path):
    return json.loads(path.read_text(encoding='utf-8'))


def trace_rows(path):
    trace = read_json(path)
    expected = trace['specIds']
    rows = trace['items']
    actual = [row['id'] for row in rows]
    if len(expected) != len(set(expected)) or actual != expected:
        raise ValueError(f'Trace roster differs from its declared identifiers in {path}')
    kinds = Counter(row['realisation']['kind'] for row in rows)
    accepted = ('check', 'data', 'design-case', 'not-modelled')
    if kinds.keys() - set(accepted):
        raise ValueError(f'Unknown realisation kinds in {path}: {sorted(kinds.keys() - set(accepted))}')
    return [len(expected), *(kinds[tag] for tag in accepted)], [
        row['id'] for row in rows if row['realisation']['kind'] == 'not-modelled'
    ]


def main():
    root = Path(__file__).resolve().parents[7]
    norm = root / '✏️s/🔌️plugins/📕️norm'
    artifacts = {}
    for path in (norm / '🗿️artifacts').glob('*/📜️artifact-definition.json'):
        artifact = read_json(path)['artifact']
        if artifact in artifacts:
            raise ValueError(f'Duplicate norm artifact {artifact}')
        artifacts[artifact] = path.parent
    expected = {*(f'en{number}' for number in range(1990, 2000)), 'din4108', 'din16798', 'din18599', 'iso16757', 'vdi3805'}
    if set(artifacts) != expected:
        raise ValueError(f'Norm artifact catalogue changed: {sorted(set(artifacts) ^ expected)}')

    print('| Family | Target IDs | Check | Data | Design case | Not modelled | Missing IDs |')
    print('| --- | ---: | ---: | ---: | ---: | ---: | --- |')
    total = [0] * 5
    for artifact, directory in sorted(artifacts.items()):
        sources = list(directory.glob('🏅️standards/*/🪆️subsets/*/🧭️spec-trace/🔣️.json'))
        if not sources:
            print(f'| {artifact} | — | — | — | — | — | Trace absent |')
            continue
        if len(sources) != 1:
            raise ValueError(f'Multiple trace rosters for {artifact}')
        counts, gaps = trace_rows(sources[0])
        total = [old + value for old, value in zip(total, counts)]
        print('|', artifact, '|', ' | '.join(map(str, counts)), '|', ', '.join(gaps) or '—', '|')
    print('| **Total** |', ' | '.join(map(str, total)), '| |')

    parameters = read_json(norm / '⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json')['entries']
    counts = Counter()
    for row in parameters:
        per_annex = row.get('annexStatus', {})
        verified = tuple(per_annex.get(side, {}).get('status', row['status']) == 'verified' for side in ('en', 'naDe'))
        if any(confirmed and row[field] is None for confirmed, field in zip(verified, ('recommended', 'naDe'))):
            raise ValueError(f"Verified annex has no value in {row['id']}")
        label = {(False, False): 'neither', (True, False): 'EN only', (False, True): 'NA-DE only', (True, True): 'both'}[verified]
        counts[label] += 1
    print('\nNational parameter verification flags:', json.dumps(dict(sorted(counts.items()))))


if __name__ == '__main__':
    main()
