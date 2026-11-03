import importlib.util, sys
spec = importlib.util.spec_from_file_location("fs", sys.argv[1])
fs = importlib.util.module_from_spec(spec); spec.loader.exec_module(fs)
print(fs.strip_comments(sys.argv[2]))
