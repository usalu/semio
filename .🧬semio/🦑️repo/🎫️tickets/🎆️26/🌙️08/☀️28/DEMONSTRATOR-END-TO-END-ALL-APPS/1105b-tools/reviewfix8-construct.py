import re,sys
def close_of(s,k):
    d=0
    while True:
        c=s[k]
        if c in '([{': d+=1
        elif c in ')]}':
            d-=1
            if d==0: return k
        k+=1
def split_args(s):
    out=[];d=0;cur=''
    for c in s:
        if c in '([{': d+=1
        elif c in ')]}': d-=1
        if c==',' and d==0: out.append(cur);cur=''
        else: cur+=c
    out.append(cur);return out
CALL='caller.turn_with_decoder('
for f in sys.argv[1:]:
    lines=open(f).read().split('\n');n=0
    for li,s in enumerate(lines):
        while True:
            a=s.find('admit_original_construction(cx,')
            if a<0: break
            ts=s.rfind(CALL,0,a);assert ts>=0
            te=close_of(s,ts+len(CALL)-1)
            targs=split_args(s[ts+len(CALL):te])
            grant=targs[0]
            ae=close_of(s,a+len('admit_original_construction'))
            aargs=split_args(s[a+len('admit_original_construction('):ae])
            out=aargs[1];demand=aargs[2];clo=','.join(aargs[3:])
            assert clo.startswith('|_|'),clo[:20]
            clo='|_,_|'+clo[3:]
            posted='let mut posted=None;' in s[ts:a]
            if posted: out='&mut None::<()>'
            call='crate::effect_fixture::construct_ready(&mut caller,&mut decoder,'+grant+','+demand+','+out+',&mut '+clo+')'
            # whole turn expression
            after=s[te+1:]
            # trailing unwrap for cancel site
            if s[:ts].endswith('assert!(!') and after.startswith('.unwrap());'):
                s=s[:ts-len('assert!(!')]+'assert_ne!('+call+'.0,semio_framework_job::OriginalConstructionStatus::Ready);'+after[len('.unwrap());'):]
            else:
                tail=')));' if not posted else ')}));'
                assert after.startswith(')' + (')' if True else '')) or True
                # consume the closing paren of observe closure forms: find end of the turn_with_decoder closure arg: te already is end of turn call
                s=s[:ts]+call+after
                s=s.replace('let(result,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(||'+call,'let((result,rc,rl),h)=semio_framework_trace::observe_heap_allocations_on_this_thread(||'+call)
                s=s.replace('let(result,h)=observe(||'+call,'let((result,rc,rl),h)=observe(||'+call)
                s=s.replace('assert!(result.unwrap());assert_eq!((h.requested_bytes,h.released_bytes),(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));','assert_eq!(result,semio_framework_job::OriginalConstructionStatus::Ready);assert_eq!((h.requested_bytes,h.released_bytes),(rc,rl));')
            n+=1
        lines[li]=s
    open(f,'w').write('\n'.join(lines)) if False else None
    open(f,'w').write('\n'.join(lines))
    print(f,n)
