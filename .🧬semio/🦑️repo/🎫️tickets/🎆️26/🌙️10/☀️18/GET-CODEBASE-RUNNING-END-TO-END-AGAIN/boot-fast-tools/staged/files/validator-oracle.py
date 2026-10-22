import json,sys
"""Independent reading of the shared relative path pattern as a specification, not as a regular expression: a path is accepted when it is not empty, holds no line terminator (LF, CR, U+2028, U+2029), does not start with a slash, holds no backslash and no NUL and has no segment that is exactly two dots. Reads a JSON list of strings on stdin, writes the JSON list of verdicts."""
TERMINATORS={"\n","\r","\u2028","\u2029"}
def accepted(s):
    return bool(s) and not any(c in TERMINATORS or c in "\\\0" for c in s) and not s.startswith("/") and ".." not in s.split("/")
json.dump([accepted(s) for s in json.load(sys.stdin)],sys.stdout)
