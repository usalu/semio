"""🔬 Run preview_any.py with periodic stack dumps to a file: dump_any.py <seconds> <dumpfile> <preview_any args...>"""
import faulthandler, runpy, sys
seconds, dump = float(sys.argv[1]), open(sys.argv[2], "wb", buffering=0)
faulthandler.dump_traceback_later(seconds, repeat=True, file=dump)
sys.argv = ["preview_any.py", *sys.argv[3:]]
runpy.run_path(__file__.replace("dump_any.py", "preview_any.py"), run_name="__main__")
