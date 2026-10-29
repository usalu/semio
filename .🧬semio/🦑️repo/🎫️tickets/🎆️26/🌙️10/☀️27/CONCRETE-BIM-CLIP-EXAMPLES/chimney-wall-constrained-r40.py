constrained=[]
for target_y in [11.4,11.5,11.6,11.625]:
 def residual_wall(v):
  w=v[0];f=(target_y-8.23)/(w/71-.4/41)
  return residual([f,w])
 fit=least_squares(residual_wall,[1.2],bounds=([1.0],[1.4]),max_nfev=250)
 w=fit.x[0];f=(target_y-8.23)/(w/71-.4/41);cx,cy,C=camera(f,w)
 X=C[0]-(818-cx)*.4/41;Z=C[2]+(cy-130)*.4/41
 constrained.append({'Y':target_y,'width':float(w),'f':float(f),'rms':float(np.sqrt(np.mean(residual([f,w])**2))),'cylinder':[float(X),target_y,float(Z)]})
print(json.dumps(constrained,indent=2))
(base/'🗑️generated/chimney-wall-constrained-r40.json').write_text(json.dumps(constrained,indent=2))
