"""🧫 [isolate] manual edits of batch `buildb`: the same rules as `builda` (one table for both build batches). @see ./builda.py"""
import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "builda.py"), encoding="utf-8").read())
