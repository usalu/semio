import sys,re
t=open(sys.argv[1],encoding='utf-8',errors='replace').read()
for b in re.split(r'\n(?=error)',t):
    l=b.split('\n')
    if l[0].startswith('error[') and len(l)>1 and '🏗️fem/🗿️artifacts' in l[1]:
        print('\n'.join(x[:260] for x in l[:9])); print('---')
