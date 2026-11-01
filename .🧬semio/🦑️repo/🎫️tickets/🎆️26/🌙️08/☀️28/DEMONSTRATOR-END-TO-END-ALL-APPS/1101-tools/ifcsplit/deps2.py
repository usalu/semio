import re,os,sys,collections
D="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details"
modfile={"type_metadata":"type_metadata","type_remap":"type_remap","type_input":"type_input","type_input_kind":"type_input_kind","type_input_select":"type_input_select","type_guidance":"type-guidance","candidate_rules":"candidate_rules","receive":"receive","value_edit":"value_edit","domain_work":"domain_work","domain_edit":"domain_edit","wire":"wire","source_loan":"source_loan","type_guidance_request":"type_guidance_request","type_guidance_candidate":"type_guidance_candidate","work":"work","admission":"admission","command_wire":"command_wire","pointer":"pointer","reference":"reference","contextual":"contextual","contextual_render":"contextual_render","contextual_edit":"contextual_edit","entity_remove":"entity_remove","constituent_roster":"constituent_roster","inverse":"inverse","preimage":"preimage","choice_rows":"type_input_choices","list_copy":"type_input_list_copy","typed_copy":"type_input_typed_copy","set_controls":"type_input_set_controls","set_copy":"type_input_set_copy","domain":"dom:🦀️","domain_cursor":"dom:cursor","source_index":"src:index","identity":"id"}
files=["🦀️"]+sorted(set(v for v in modfile.values() if not v.startswith(("dom:","src:","id"))))+["dom:core","dom:cursor","dom:🦀️","dom:membership","dom:ifc4"]
def path(f):
    if f.startswith("dom:"): return D+"/../📚️catalog/⚖️domain/"+f[4:]+".rs"
    return D+"/"+f+".rs"
def strip(s):
    s=re.sub(r'/\*.*?\*/','',s,flags=re.S)
    s=re.sub(r'//[^\n]*','',s)
    s=re.sub(r'"(\\.|[^"\\])*"','""',s)
    # remove cfg(test) items
    out=[];i=0
    while True:
        j=s.find("#[cfg(test)]",i)
        if j<0: out.append(s[i:]);break
        out.append(s[i:j]);k=j+12
        # skip attributes
        while True:
            m=re.match(r'\s*#\[[^\]]*\]',s[k:])
            if m:k+=m.end()
            else:break
        # find first { or ; 
        m=re.search(r'[{;]',s[k:]);
        if not m:break
        if m.group()==';':i=k+m.end();continue
        depth=0;p=k+m.start()
        while p<len(s):
            if s[p]=='{':depth+=1
            elif s[p]=='}':
                depth-=1
                if depth==0:break
            p+=1
        i=p+1
    return "".join(out)
src={f:strip(open(path(f)).read()) for f in files}
rootdefs=set(m.group(1) for m in re.finditer(r'^(?:pub(?:\([a-z]+\))?\s+)?(?:async\s+)?(?:struct|enum|trait|fn|const|type|static)\s+([A-Za-z_][A-Za-z0-9_]*)',src["🦀️"],re.M))
rootuses=set()
for m in re.finditer(r'^use ([^;]+);',src["🦀️"],re.M):
    rootuses.add(m.group(0))
for f in files:
    t=src[f]; ids=set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*',t))
    owndefs=set(m.group(1) for m in re.finditer(r'^\s*(?:pub(?:\([a-z]+\))?\s+)?(?:async\s+)?(?:struct|enum|trait|fn|const|type|static)\s+([A-Za-z_][A-Za-z0-9_]*)',t,re.M))
    sib=sorted(m for m in modfile if re.search(r'\b'+m+r'::',t) and modfile[m]!=f)
    rd=sorted(n for n in rootdefs if f!="🦀️" and n in ids and n not in owndefs)
    print("==",f,len(t),"MOD",sib,"ROOT",rd)
