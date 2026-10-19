import re, sys
for p in sys.argv[1:]:
    s = open(p, encoding='utf8').read()
    o = len(re.findall(r'(?m)^[ \t]*//\s*#region\b', s)); c = len(re.findall(r'(?m)^[ \t]*//\s*#endregion\b', s))
    print(('OK ' if o == c else 'UNBALANCED ') + p[-60:], o, c)
