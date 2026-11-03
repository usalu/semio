import sys
sys.argv=["x"]
exec(open("audit.py").read().split('if __name__')[0])
entries, vocab = extract_entries("🪵️en1995"), extract_vocabulary("🪵️en1995")
need = {"T-90":["a1","i1","s1","a2","i2","h1","h2"],"T-93":["n_parts","area","lef","l1"],"T-94":["b_i","s","n_panels","h","l_dia"],"T-95":["span","height","bays","chord_b","chord_h","web_b","web_h"],"T-96":["n_braced","spacing","member_b","member_h","l_m"],"T-101":["t_req","face_left","face_right","face_top","face_bottom"],"T-102":["t_req","face_left"],"T-106":["t_req","face_left"],"T-107":["t_req","n_layers"],"T-120":["crowd_density"],"T-121":["n1","n2","years","ds1"],"T-122":["bar_spacing","plate_a","contact_mm","deck_depth"]}
for e,keys in need.items():
    inputs = entries[e][1]
    print(e, {k:(k in inputs, (vocab_spec(vocab,e,k) or {}).get("unit")) for k in keys})
