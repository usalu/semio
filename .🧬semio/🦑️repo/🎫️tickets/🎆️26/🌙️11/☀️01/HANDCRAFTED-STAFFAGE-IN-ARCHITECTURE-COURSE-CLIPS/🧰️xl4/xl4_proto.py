import xl4_common as C
v = C.load()
p = v._engine.VMobject()
for k, x in vars(p).items():
    print(k, type(x).__name__, repr(x)[:50])
