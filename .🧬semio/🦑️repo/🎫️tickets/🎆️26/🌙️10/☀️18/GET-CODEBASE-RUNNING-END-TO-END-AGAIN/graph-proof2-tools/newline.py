import hashlib, sys
path, mode = sys.argv[1], sys.argv[2]
data = open(path, "rb").read()
print("sha", hashlib.sha256(data).hexdigest(), "len", len(data), "last", data[-1:])
if mode == "add":
    assert hashlib.sha256(data).hexdigest() == "33ce24a402abb44b1ecec83e850e4a450cbca3058939983b17cfa36f63b029a0", "pre-sha mismatch"
    open(path, "wb").write(data + b"\n")
elif mode == "restore":
    assert len(data) == 6045 and data[-1:] == b"\n", "not the newline state"
    open(path, "wb").write(data[:-1])
