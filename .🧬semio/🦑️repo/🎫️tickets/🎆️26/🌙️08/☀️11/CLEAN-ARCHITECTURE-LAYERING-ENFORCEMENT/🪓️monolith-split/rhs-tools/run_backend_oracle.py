import sys,os,json,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from sandbox import *
import backend_split
R_='🧰️framework/🛍️products/🦑️repo/🔨️modules/'
SPECS={
 'contract':dict(mirror='🌎️hub/📇️directory',target='🌎️hub/📇️directory/📜️contract',run_dir='🌎️hub/📇️directory/📜️contract',root='🦀️.rs',root_file='🦀️.rs',crate='semio_hub_directory_contract',orig='contract',skip=['📦️packages']),
 'rows':dict(mirror='🌎️hub/📇️directory/🌐️neo4j',target='🌎️hub/📇️directory/🌐️neo4j/🧰️rows',run_dir='🌎️hub/📇️directory/🌐️neo4j/🧰️rows',root='🦀️.rs',root_file='🦀️.rs',crate='semio_hub_directory_neo4j_rows',orig='neo4j-rows',skip=['📦️packages']),
 'model':dict(mirror=R_+'📐️model',target=R_+'📐️model/📦️packages/🦀️rust',run_dir=R_+'📐️model/📦️packages/🦀️rust',root='🦀️.rs',root_file='🦀️.rs',crate='semio_framework_repo_model',orig='model'),
 'tickets':dict(mirror=R_+'🎫️tickets',target=R_+'🎫️tickets/📦️packages/🦀️rust',run_dir=R_+'🎫️tickets/📦️packages/🦀️rust',root='🦀️.rs',root_file='🦀️.rs',crate='semio_framework_repo_tickets',orig='tickets'),
 'gateway':dict(mirror='🧰️framework/🛍️products/🖥️server',target='🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway',run_dir='🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust',root='🦀️.rs',root_file='🦀️.rs',crate='server',orig='gateway',skip=[]),
 'sqlite':dict(real_dir='🌎️hub/📇️directory/🪶️sqlite',root='🦀️.rs',root_file='🦀️.rs',crate='semio_hub_directory_sqlite'),
 'postgres':dict(real_dir='🌎️hub/📇️directory/🐘️postgres',root='🦀️.rs',root_file='🦀️.rs',crate='semio_hub_directory_postgres'),
 'neo4j':dict(real_dir='🌎️hub/📇️directory/🌐️neo4j',root='🦀️.rs',root_file='🦀️.rs',crate='semio_hub_directory_neo4j'),
}
CONFIGS={'conformance':['--cfg','feature="conformance"'],'test+conformance':['--test','--cfg','feature="conformance"'],'native':[], 'test':['--test'], 'testkit':['--cfg','feature="testkit"'], 'test+testkit':['--test','--cfg','feature="testkit"']}
def main(backend,stage,configs):
    spec=SPECS[backend]
    base=build(spec,backend+'-base',orig_file=os.path.join(T,'orig/%s.rs.txt'%spec.get('orig',backend)))
    split=build(spec,backend+'-split',variant_dir=None if stage=='REAL' else stage)
    out=os.path.join(SC,'rhs-orc')
    res={}
    for cn in configs:
        extra=CONFIGS[cn]
        a=norm(run(base,spec,backend+'-base-'+cn,extra,out),base)
        b=norm(run(split,spec,backend+'-split-'+cn,extra,out),split)
        ma,mb=compare(a,b)
        print('%s %s: base %d split %d only-base %d only-split %d'%(backend,cn,sum(a.values()),sum(b.values()),sum(ma.values()),sum(mb.values())))
        for k,v in sorted(ma.items()):print('  -',v,k[:240])
        for k,v in sorted(mb.items()):print('  +',v,k[:240])
if __name__=='__main__':
    main(sys.argv[1],sys.argv[2],sys.argv[3:] or ['native','test'])
