"""🌊️ The shared streaming reader of the ticket logs: `rg` pipes matching lines per file through a generator (never the whole tree or a whole log in memory) and the process aborts above 500 MB RSS. Used by the capacity trace (`trace.py`) and open to the lease queue extractor (`FOR lease-g`): `for path, lines in stream_by_file(r"Waiting for access")`."""
import itertools,os,resource,subprocess,sys
LIMIT=500*1024*1024
ROOT=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GENERATED=os.path.join(ROOT,"🗑️generated")
def guard():
    peak=max(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss)
    if peak>(LIMIT if sys.platform=="darwin" else LIMIT//1024): sys.exit("rss cap exceeded: 500 MB")
def stream(pattern,exclude=("ram-g",)):
    command=["rg","-I","--no-heading","--with-filename","--no-line-number","--no-messages","--no-mmap","--max-filesize","100M","-j","2",pattern,GENERATED]+[a for e in exclude for a in ("--glob","!"+e+"/**")]
    process=subprocess.Popen(command,stdout=subprocess.PIPE,text=True,errors="replace",bufsize=1<<16)
    try:
        for count,raw in enumerate(process.stdout):
            if count%5000==0: guard()
            path,separator,text=raw.rstrip("\n").partition(":")
            if separator: yield path,text
    finally:
        process.kill();process.wait()
def stream_by_file(pattern,exclude=("ram-g",)):
    for path,group in itertools.groupby(stream(pattern,exclude),key=lambda row:row[0]):
        yield path,(text for _,text in group)
