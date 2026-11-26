import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from f72lib import *
if have():print("already mine")
else:
    g=take(100);print("got" if g else "held",open(f"{TICKET}/landing.lock.caching-cargo/owner").read())
