from pathlib import Path
import json

base=Path('🎓️teaching/🏛️architecture')
for folder in ['🏗️bim','📐️parametric']:
    path=base/folder/'❓️quiz/🔣️.json'
    quiz=json.loads(path.read_text(encoding='utf-8'))
    fixture=json.loads((base/'📚️course/🧫️fixtures'/folder/'🔣️.json').read_text(encoding='utf-8'))
    assert quiz['id']==fixture['quiz']
    assert (path.parent/quiz['$schema'].split('#')[0]).resolve().exists()
    assert [{'id':t['id'],'kind':t['kind']} for t in quiz['tasks']]==fixture['tasks']
    tasks={t['id']:t for t in quiz['tasks']}
    count=0
    for task,expected in fixture['classifications'].items():
        actual={i['id']:i['category'] for i in tasks[task]['items']}
        for item,category in expected.items():
            assert actual[item]==category,(task,item)
            count+=1
    if 'axes' in fixture:
        profiles=tasks['design-tool-profiles']
        assert [a['id'] for a in profiles['axes']]==fixture['axes']
        assert {c['id']:c['profile'] for c in profiles['categories']}==fixture['profiles']
    print('[DEBUG] '+quiz['id']+': '+str(count)+' independent classification expectations passed.')
