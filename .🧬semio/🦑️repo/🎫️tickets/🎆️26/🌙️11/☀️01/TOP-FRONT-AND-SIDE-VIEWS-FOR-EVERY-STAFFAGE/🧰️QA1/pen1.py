import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
def edit(aid,sc,ids,en,de):
    f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+aid+'/🔣️.json')[0]
    d=json.load(open(f,encoding='utf8')); r=d['asset']['representations'][sc]
    n=0
    for s in r['geometry']['front']:
        if s['id'] in ids: s['penMm']=0.2; n+=1
    t=r['decisions']['silhouette']; t['en']+=' '+en; t['de']+=' '+de
    write_json_atomic(f,d); print(aid,sc,n)
edit('domestic-cat','200',{'tail-1','tail-2','tail-3','tail-4','tail-5','tail-tip'},'In front the tail pieces use a 0.2 mm pen: a piece hidden behind the body in a pose (play bow, stretch) is still measured at its pen width.','In der Vorderansicht zeichnen die Schweifstücke mit 0,2 mm Feder: Ein in einer Pose (Spielverbeugung, Strecken) hinter dem Körper verdecktes Stück wird weiter mit seiner Federbreite gemessen.')
edit('labrador-dog','200',{'near-hind-thigh','far-hind-thigh'},'In front the hind thighs use a 0.2 mm pen: a thigh hidden behind the trunk when sitting is still measured at its pen width.','In der Vorderansicht zeichnen die Hinterschenkel mit 0,2 mm Feder: Ein beim Sitzen hinter dem Rumpf verdeckter Schenkel wird weiter mit seiner Federbreite gemessen.')
edit('labrador-dog','500',{'near-hind-thigh','far-hind-thigh'},'In front the hind thighs use a 0.2 mm pen: a thigh hidden behind the trunk when sitting is still measured at its pen width.','In der Vorderansicht zeichnen die Hinterschenkel mit 0,2 mm Feder: Ein beim Sitzen hinter dem Rumpf verdeckter Schenkel wird weiter mit seiner Federbreite gemessen.')
edit('dairy-cow','500',{'udder'},'In front the udder uses a 0.2 mm pen so that the small udder of a lying cow stays above the minimum feature.','In der Vorderansicht zeichnet das Euter mit 0,2 mm Feder, damit das kleine Euter einer liegenden Kuh über dem Mindestmerkmal bleibt.')
