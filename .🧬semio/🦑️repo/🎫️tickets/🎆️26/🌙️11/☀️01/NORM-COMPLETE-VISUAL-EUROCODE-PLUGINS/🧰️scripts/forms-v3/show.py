import sys
exec(open("audit.py").read().split('if __name__')[0])
code=sys.argv[1]
entries, vocab = extract_entries(code), extract_vocabulary(code)
for e in sys.argv[2:]:
    print(e, entries[e][0], [(k,(vocab_spec(vocab,e,k) or {}).get('unit')) for k in entries[e][1]])
