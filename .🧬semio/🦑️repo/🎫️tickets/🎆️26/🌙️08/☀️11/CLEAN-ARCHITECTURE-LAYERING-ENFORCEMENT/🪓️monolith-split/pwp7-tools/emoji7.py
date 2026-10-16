import sys; sys.path.insert(0,'.')
from ext7 import *
CANDS='🍀🍁🍂🍃🌰🌵🌶🌽🌾🌿🍄🌻🌼🌷🌹🌺🌸🌴🌳🌲🌱🌟🌠🌈🌋🌌🌍🌎🌏🌑🌒🌓🌔🌕🌖🌗🌘🌙🌚🌛🌜🌝🌞🌡🌪🌫🌬🌭🌮🌯🌰🌱🍇🍈🍉🍊🍋🍌🍍🍎🍏🍐🍑🍒🍓🥝🥥🥭🍔🍕🍖🍗🍘🍙🍚🍛🍜🍝🍞🍟🍠🍡🍢🍣🍤🍥🍦🍧🍨🍩🍪🍫🍬🍭🍮🍯🍰🍱🍲🍳🍴🍵🍶🍷🍸🍹🍺🍻🍼🎀🎁🎂🎃🎄🎅🎆🎇🎈🎉🎊🎋🎌🎍🎎🎏🎐🎑🎒🎓🎠🎡🎢🎣🎤🎥🎦🎧🎨🎩🎪🎫🎬🎭🎮🎯🎰🎱🎲🎳🎴🎵🎶🎷🎸🎹🎺🎻🎼🎽🎾🎿🏀🏁🏂🏃🏄🏅🏆🏇🏈🏉🏊🏏🏐🏑🏒🏓🏸🏹🏺🐀🐁🐂🐃🐄🐅🐆🐇🐈🐉🐊🐋🐌🐍🐎🐏🐐🐑🐒🐓🐔🐕🐖🐗🐘🐙🐚🐛🐜🐝🐞🐟🐠🐡🐢🐣🐤🐥🐦🐧🐨🐩🐪🐫🐬🐭🐮🐯🐰🐱🐲🐳🐴🐵🐶🐷🐸🐹🐺🐻🐼🐽🐾🦀🦁🦂🦃🦄🦅🦆🦇🦈🦉🦊🦋🦌🦍🦎🦏🦐🦑🦒🦓🦔🦕🦖🦗🦘🦙🦚🦛🦜🦝🦞🦟🦠🦡🦢🦥🦦🦧🦨🦩🦪🦮🧁🧂🧃🧄🧅🧆🧇🧈🧉🧊🧋'
def used():
    files=plugin_files()
    s=set()
    for f in files:
        try:
            with open(ROOT+f,encoding='utf-8') as fh: first=[l for l in (fh.readline() for _ in range(3))]
        except Exception: continue
        for l in first:
            m=re.match(r'//!\s*(\S+)',l)
            if m: s.add(m.group(1).replace('️','')); break
    return s
def pool(n):
    u=used()
    out=[]
    for ch in CANDS:
        if ch not in u and ch not in out: out.append(ch)
    return out
if __name__=='__main__':
    u=used(); print(len(u)); p=pool(0); print(len(p), p[:40])
