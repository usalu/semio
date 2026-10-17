import re,sys
for p in sys.argv[1:]:
    t=open(p,encoding="utf8").read()
    hits=[m.start() for m in re.finditer(r"\*/\n\n(?=(?:export |function |const |class |interface |type |async ))",t)]
    print(len(hits),p.split("/❓️quiz/")[-1])
