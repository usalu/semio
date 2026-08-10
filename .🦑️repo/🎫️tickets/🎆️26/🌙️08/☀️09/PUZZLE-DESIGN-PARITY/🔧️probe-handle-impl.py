from pathlib import Path
root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
for label in ['2d','3d']:
    appdir=next(p for p in next(root.glob('*apps*')).iterdir() if p.name.endswith(label))
    t=(appdir/'🦀️component.rs').read_text()
    # find fn handle(
    idx=t.find('fn handle(')
    # might be multiple - find DocumentApp impl handle
    idxs=[]
    start=0
    while True:
        i=t.find('fn handle(', start)
        if i<0: break
        idxs.append(i)
        start=i+1
    print(label, 'handle count', len(idxs))
    for i in idxs:
        print('\n====', label, 'handle at', i)
        print(t[i:i+3500])
        print('----')
