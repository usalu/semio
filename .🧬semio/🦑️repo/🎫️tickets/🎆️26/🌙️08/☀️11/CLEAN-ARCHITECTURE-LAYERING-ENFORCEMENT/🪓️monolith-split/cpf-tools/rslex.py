import re
TOK=re.compile(r'''
  (?P<lc>//[^\n]*)
 |(?P<bc>/\*)
 |(?P<raw>(?:b|c)?r(?P<h>\#*)")
 |(?P<str>(?:b|c)?"(?:[^"\\]|\\.)*")
 |(?P<chr>b?'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]+\}|.)|[^'\\\n])')
 |(?P<life>'[A-Za-z_][A-Za-z0-9_]*)
 |(?P<id>[A-Za-z_][A-Za-z0-9_]*)
 |(?P<p>[{}()\[\];])
''',re.X|re.S)
DEFKW={"impl","trait","struct","enum","union"}
def lex(text):
    """returns (comments, ok). comment: dict(start,end,block,doc,ctx)"""
    comments=[]; pos=0; n=len(text)
    stack=[]  # entries: 'fn','blk','def','mod','other'
    pdepth=0; pending_fn=None; item_kw=None; prev_id=None
    def in_fn(): return 'fn' in stack
    def in_def(): return bool(stack) and stack[-1]=='def'
    while pos<n:
        m=TOK.search(text,pos)
        if not m: break
        k=m.lastgroup
        if k=='h': k='raw'
        s=m.start()
        if m.group('lc') is not None:
            c=m.group('lc'); doc=c.startswith('///') and not c.startswith('////') or c.startswith('//!')
            comments.append(dict(start=s,end=m.end(),block=False,doc=doc,fn=in_fn(),de=in_def(),text=c))
            pos=m.end(); continue
        if m.group('bc') is not None:
            depth=1; j=m.end()
            while j<n and depth:
                a=text.find('/*',j); b=text.find('*/',j)
                if b<0: j=n; break
                if a>=0 and a<b: depth+=1; j=a+2
                else: depth-=1; j=b+2
            c=text[s:j]; doc=(c.startswith('/**') and not c.startswith('/***') and not c.startswith('/**/')) or c.startswith('/*!')
            comments.append(dict(start=s,end=j,block=True,doc=doc,fn=in_fn(),de=in_def(),text=c))
            pos=j; continue
        if m.group('raw') is not None:
            h=m.group('h'); close='"'+h
            j=text.find(close,m.end())
            pos=n if j<0 else j+len(close); continue
        if m.group('str') is not None or m.group('chr') is not None or m.group('life') is not None:
            pos=m.end(); continue
        if m.group('id') is not None:
            w=m.group('id')
            if w=='fn' and pending_fn is None and not text[m.end():m.end()+40].lstrip().startswith('('):
                pending_fn=pdepth
            elif w in DEFKW or w in ('mod','macro_rules'):
                if pending_fn is None: item_kw=w
            prev_id=w
            pos=m.end(); continue
        ch=m.group('p')
        if ch in '([': pdepth+=1
        elif ch in ')]': pdepth=max(0,pdepth-1)
        elif ch=='{':
            if pending_fn is not None and pdepth==pending_fn:
                stack.append('fn'); pending_fn=None; item_kw=None
            elif in_fn(): stack.append('blk')
            elif item_kw in DEFKW: stack.append('def'); item_kw=None
            elif item_kw=='mod': stack.append('mod'); item_kw=None
            else: stack.append('other'); item_kw=None
        elif ch=='}':
            if stack: stack.pop()
            item_kw=None; pending_fn=None
        elif ch==';':
            if pending_fn is not None and pdepth==pending_fn: pending_fn=None
            if pdepth==0: item_kw=None
        pos=m.end()
    return comments, (not stack)
def strip_comments(text):
    """text with all comments removed and whitespace collapsed (code equality check)."""
    cs,_=lex(text); out=[]; last=0
    for c in cs:
        out.append(text[last:c['start']]); last=c['end']
    out.append(text[last:])
    return re.sub(r"\s+"," ","".join(out)).strip()
