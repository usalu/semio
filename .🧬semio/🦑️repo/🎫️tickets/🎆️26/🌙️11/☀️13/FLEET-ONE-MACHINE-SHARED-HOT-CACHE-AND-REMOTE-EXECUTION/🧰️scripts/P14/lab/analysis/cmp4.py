import pickle,json,collections,re,hashlib
out=pickle.load(open('p3/dec.pkl','rb'))
dec=json.JSONDecoder()
print(json.dumps(dec.raw_decode(out[5][1].decode())[0]['observations'][:6],ensure_ascii=False)[:900])
