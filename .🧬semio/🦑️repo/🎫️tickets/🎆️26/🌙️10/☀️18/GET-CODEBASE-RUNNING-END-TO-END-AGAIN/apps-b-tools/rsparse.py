import re
def skip_str(s,i):
    # at s[i]=='"' -> index after string
    j=i+1
    while j<len(s):
        if s[j]=='\\': j+=2; continue
        if s[j]=='"': return j+1
        j+=1
    return j
def match_brace(s,i):
    """s[i] is an opening bracket; return index of matching close, skipping strings/chars/comments."""
    pairs={'{':'}','(':')','[':']'}
    stack=[pairs[s[i]]]
    j=i+1
    n=len(s)
    while j<n and stack:
        c=s[j]
        if c=='/' and s.startswith('//',j):
            k=s.find('\n',j); j=n if k<0 else k; continue
        if c=='/' and s.startswith('/*',j):
            k=s.find('*/',j+2); j=n if k<0 else k+2; continue
        if c=='"': j=skip_str(s,j); continue
        if c=='r' and re.match(r'r#*"',s[j:j+6]) and (j==0 or not (s[j-1].isalnum() or s[j-1]=='_')):
            m=re.match(r'r(#*)"',s[j:]); h=m.group(1); k=s.find('"'+h,j+len(m.group(0)))
            j=n if k<0 else k+1+len(h); continue
        if c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",s[j:])
            if m: j+=len(m.group(0)); continue
            j+=1; continue
        if c in pairs: stack.append(pairs[c]); j+=1; continue
        if c==stack[-1]:
            stack.pop(); j+=1
            if not stack: return j-1
            continue
        j+=1
    return -1
