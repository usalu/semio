"""🔎 Recheck every retained uncertain snapshot pair with exact sufficient negatives, never current-model acceptance."""
import hashlib,importlib.util,json,sys,time,traceback
from pathlib import Path
sys.dont_write_bytecode=True
T=Path(__file__).parent;G=T/'🗑️generated';D=G/'whole-diagnostic-r95';paths=[T/'whole-expanded-interference-r92.py',Path(__file__),T/'fixture-observer-r95.py',D/'whole-expanded-owned-capture-r92.json',D/'whole-expanded-interference-result-r92.json',G/'whole-exact-separation-snapshot-baseline-r95.json']
def hashes():
    """🔐 Bind the immutable historic operands and the current exact certificate implementation."""
    return{p.name:hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
before=hashes();spec=importlib.util.spec_from_file_location('snapshot_support_r95',paths[0]);A=importlib.util.module_from_spec(spec);spec.loader.exec_module(A);ospec=importlib.util.spec_from_file_location('snapshot_observer_r95',paths[2]);O=importlib.util.module_from_spec(ospec);ospec.loader.exec_module(O);observer=O.Observer(T,'uncertain-snapshot');start=time.monotonic();capture=json.loads(paths[3].read_text(encoding='utf8'));prior=json.loads(paths[4].read_text(encoding='utf8'));records={(r['tag'],r['index']):r for r in capture['records']};solids={};rows=[];counts={'exact_negative':0,'unproven':0,'unsupported':0}
try:
    for index,pair in enumerate(prior['unresolved_part_pairs']):
        row={'prior_index':index,'source':pair['source'],'target':pair['target']}
        try:
            values=[]
            for side in('source','target'):
                r=pair[side];key=(r['tag'],r['index'])
                if key not in solids:solids[key]=A.Solid(records[key])
                values.append(solids[key])
            proof=A.exact_support_separation(*values);row.update(classification='exact_negative'if proof else'unproven',proof=proof)
        except Exception:row.update(classification='unsupported',error=traceback.format_exc())
        counts[row['classification']]+=1;rows.append(row)
        if index%250==0:print('[DEBUG] snapshot support '+json.dumps({'completed':index+1,'total':len(prior['unresolved_part_pairs']),'seconds':time.monotonic()-start,**counts}),flush=True)
finally:observer.close()
after=hashes();baseline=json.loads(paths[5].read_text(encoding='utf8'));expected=[(r['prior_index'],r['classification'],r.get('proof'))for r in baseline['rows']];actual=[(r['prior_index'],r['classification'],r.get('proof'))for r in rows];assert actual==expected,'Exact integer-lattice classifications or certificates changed';out={'same_classifications_and_certificates_as_fraction_baseline':True,'baseline_seconds':baseline['seconds'],'status':'DIAGNOSTIC_SNAPSHOT_ONLY','source':capture['source'],'current_source_acceptance':False,'stable':before==after,'before':before,'after':after,'pairs':len(rows),'counts':counts,'rows':rows,'seconds':time.monotonic()-start,'scope':'Every original uncertain pair is retained as an exact negative, still unproved or unsupported. Historic operands only; source/producer changes prohibit any current or final model clearance claim.'};assert len(rows)==len(prior['unresolved_part_pairs']);(G/'whole-exact-separation-snapshot-r95.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items()if k not in('rows','before','after')}),flush=True);raise SystemExit(not out['stable'])
