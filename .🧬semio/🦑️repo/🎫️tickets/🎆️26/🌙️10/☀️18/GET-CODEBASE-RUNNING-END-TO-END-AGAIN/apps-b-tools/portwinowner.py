#!/usr/bin/env python3
"""Ports `WindowConfigOwner::build_store_owners()` to the funded form (preparation_source_birth + build_store_owners(grant)). Idempotent."""
import os,re,sys
sys.path.insert(0,os.path.dirname(__file__))
from rsparse import match_brace
mine=sys.argv[1:] if len(sys.argv)>1 and not sys.argv[1].startswith('--') else None
os.chdir('/Users/ueli/Documents/semio')
PLUG=['➗️mathematical','🗒️note','📕️norm','📖️playbook','🌀️procedural','🏭️process','🧩️puzzle','🖨️raster','💡️reasoning','📸️remodel','🎬️sequence','🎥️shooting','🪵️sourcing','🪐️space','🗄️stdio','🔱️trinity','🌿️vcs','🀄️wfc','✒️writer']
NEW='''fn preparation_source_birth() -> semio_framework_value::retained_clone::RetainedCloneBirthDemand { semio_framework_plugin::bounded_window_config_preparation_source_birth::<Self>() }
{I}fn build_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_store_owners::<Self>(grant) }'''
n=0;total=0
for p in PLUG:
    for r,ds,fs in os.walk('✏️s/🔌️plugins/'+p):
        ds[:]=[d for d in ds if d not in('node_modules','target')]
        for f in fs:
            if not f.endswith('.rs'): continue
            fp=os.path.join(r,f)
            try:s=open(fp,encoding='utf-8').read()
            except:continue
            if 'WindowConfigOwner for' not in s: continue
            out=[];pos=0;ch=False
            for m in re.finditer(r'impl\s+(?:[\w:]*::)?WindowConfigOwner\s+for\s+[\w$]+\s*\{',s):
                ob=m.end()-1;eb=match_brace(s,ob);body=s[ob:eb]
                fm=re.search(r'^([ \t]*)fn build_store_owners\(\)',body,re.M)
                if not fm: continue
                st=fm.start()+len(fm.group(1)); ind=fm.group(1)
                ob2=body.find('{',body.find(')',st)); 
                # the signature return type may contain braces? no. find body open after '->' ... use first '{' after signature end '>' pattern
                sig_end=body.find('{',st)
                e2=match_brace(body,sig_end)
                nb=body[:fm.start()]+ind+NEW.replace('{I}',ind)+body[e2+1:]
                out.append(s[pos:ob]);out.append(nb);pos=eb;ch=True;total+=1
            out.append(s[pos:])
            if ch:
                open(fp,'w',encoding='utf-8').write(''.join(out));n+=1
print('files',n,'impls',total)
