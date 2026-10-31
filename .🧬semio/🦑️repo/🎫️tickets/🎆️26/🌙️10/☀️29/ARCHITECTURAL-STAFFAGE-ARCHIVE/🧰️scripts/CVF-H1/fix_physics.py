"""CVF-H1 data fixes (thermal/transport defects): nose jets 40 degrees below the face direction (no backward component in leaning/rising poses), elderly mouth route."""
import sys
sys.path.insert(0, '/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CVF-H1/h1')
from lib import load, save, T

for aid in ['walking-commuter', 'seated-reader', 'running-child', 'wheelchair-user', 'elderly-with-cane', 'cyclist']:
    doc = load(aid)
    a = doc['asset']
    for e in a['thermal']['emitters']:
        if e['kind'] == 'breath' and e['directionDeg'] == -150:
            e['directionDeg'] = -130
    if aid == 'elderly-with-cane':
        pats = a['thermal']['respiration']['patterns']
        if not any(p['id'] == 'recovery' for p in pats):
            pats.append({
                "id": "recovery", "title": T("Recovery breathing after walking", "Erholungsatmung nach dem Gehen"),
                "rateBpm": 20, "tidalVolumeL": 0.75, "inhaleFraction": 0.4, "pauseAfterInhaleFraction": 0.02,
                "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.08, "route": "nose-and-mouth",
                "source": "authored-estimate", "reference": "gupta-2010"})
        clip = next(c for c in a['clips'] if c['id'] == 'rest-on-cane')
        tr = next(t for t in clip['tracks'] if t['target'] == 'breathing')
        tr['keys'] = [{"t": 0, "v": 0}, {"t": 1.6, "v": 2}, {"t": 4.2, "v": 0}]
    save(aid, doc)
    print('ok', aid)
