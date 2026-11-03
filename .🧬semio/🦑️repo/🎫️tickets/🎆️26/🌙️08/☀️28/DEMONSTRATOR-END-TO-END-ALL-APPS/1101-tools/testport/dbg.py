import sys,re,json
mode=sys.argv[1]
files=["🔗️read/📖️derived-snapshots-replays-and-history-read-plan/🦀️.rs","🔁️replay/🔢️prefix-digests-replay-progress-and-edit-replay/🦀️.rs","🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🦀️.rs"]
state={}
for f in files:
    s=open(f).read()
    if mode=='add':
        state[f]=s
        n=[0]
        def rep(m):
            n[0]+=1
            return '{eprintln!("[DEBUG] HistoryReplaying %d %s");VcsError::HistoryReplaying}'%(n[0],f[:6])
        s2=re.sub(r'\.ok_or\(VcsError::HistoryReplaying\)',lambda m:'.ok_or_else(||VcsError::HistoryReplaying)',s)
        s2=re.sub(r'VcsError::HistoryReplaying',lambda m:rep(m),s2)
        s2=s2.replace('.ok_or_else(||{eprintln','.ok_or_else(||{eprintln')
        s2=s2.replace('if !self.finished || self.retiring {','if !self.finished || self.retiring {eprintln!("[DEBUG] finish finished={} retiring={}",self.finished,self.retiring);',1)
        open(f,'w').write(s2)
    else:
        s=re.sub(r'eprintln!\("\[DEBUG\] finish[^;]*;','',s)
        s=re.sub(r'\{eprintln!\("\[DEBUG\] HistoryReplaying \d+ [^"]*"\);VcsError::HistoryReplaying\}','VcsError::HistoryReplaying',s)
        open(f,'w').write(s)
json.dump(state,open('/tmp/dbg_orig.json','w'))
