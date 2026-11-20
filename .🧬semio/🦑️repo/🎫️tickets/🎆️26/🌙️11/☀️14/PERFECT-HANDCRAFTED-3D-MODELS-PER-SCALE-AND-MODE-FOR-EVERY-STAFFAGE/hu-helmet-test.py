"""🚲 Verifies that the helmet shell and liner envelopes end at the rim, without a flat surface through the head."""
from pathlib import Path
import json
for slug in ('child-on-kick-scooter','cyclist'):
    path=next(Path.cwd().glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/'+slug+'/*'))
    for scale,volume in json.loads(path.read_text())['asset']['volumes'].items():
        for part in volume['thermal']:
            if part.get('of')=='head-helmet':continue
            if part['id'] in ('th-helmet','th-helmet-hm-shell','th-helmet-hm-air','th-helmet-hm-eps','th-helmet-hm-hair','th-helmet-helmet-air'):
                assert part.get('capStart') is False or part.get('sectionFaces')==['-u'],slug+' '+scale+' has a disc across the skull'
