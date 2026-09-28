semio norm.en1998.dsl v1
annex=de site=seismic-zone=zone2 a-gr=0.6m/s2 de-ground-combo=B-R en-ground-type="" en-spectrum-type="" importance-class=II
buildings [id:TEXT name:TEXT plan-width-m:QTY plan-length-m:QTY systems:TABLE storeys:TABLE members:TABLE plan-regular:BOOL elevation-regular:BOOL t1-method:TEXT t1-given-s:NUM ct:NUM drift-limit-class:TEXT nu:NUM multiple-resisting-systems:BOOL claims-simple-masonry:BOOL masonry-wall-area-ratio:NUM accidental-eccentricity-ratio:NUM] {
  bldg-office "Office RC frame" 24m 16m [ {id=sys-x direction=x system-type=frame material=rc ductility-class=dch q0=4.5 alpha-u-over-alpha-1=1.3 k-w=1 base-shear-resistance-n=1200000N} {id=sys-y direction=y system-type=frame material=rc ductility-class=dch q0=4.5 alpha-u-over-alpha-1=1.3 k-w=1 base-shear-resistance-n=1200000N} ] [ {id=s1 height-m=3.5m permanent-gk-n=2334780N correlated-occupancy=true stiffness-x=180000000 stiffness-y=180000000 centre-of-mass-x-m=12m centre-of-mass-y-m=8m centre-of-stiffness-x-m=12m centre-of-stiffness-y-m=8m drift-x-m=0.008m drift-y-m=0.008m shear-resistance-n=400000N
  variables [id:TEXT category:TEXT qk-n:QTY] {
    s1-q B 1716750N
  }
  } {id=s2 height-m=3.5m permanent-gk-n=2251395N correlated-occupancy=true stiffness-x=170000000 stiffness-y=170000000 centre-of-mass-x-m=12m centre-of-mass-y-m=8m centre-of-stiffness-x-m=12m centre-of-stiffness-y-m=8m drift-x-m=0.009m drift-y-m=0.009m shear-resistance-n=400000N
  variables [id:TEXT category:TEXT qk-n:QTY] {
    s2-q B 1655437.5N
  }
  } {id=s3 height-m=3.5m permanent-gk-n=2168010N correlated-occupancy=true stiffness-x=160000000 stiffness-y=160000000 centre-of-mass-x-m=12m centre-of-mass-y-m=8m centre-of-stiffness-x-m=12m centre-of-stiffness-y-m=8m drift-x-m=0.01m drift-y-m=0.01m shear-resistance-n=400000N
  variables [id:TEXT category:TEXT qk-n:QTY] {
    s3-q B 1594125N
  }
  } {id=s4 height-m=3.5m permanent-gk-n=2001240N correlated-occupancy=true stiffness-x=150000000 stiffness-y=150000000 centre-of-mass-x-m=12m centre-of-mass-y-m=8m centre-of-stiffness-x-m=12m centre-of-stiffness-y-m=8m drift-x-m=0.011m drift-y-m=0.011m shear-resistance-n=400000N
  variables [id:TEXT category:TEXT qk-n:QTY] {
    s4-q B 1177200N
  }
  } ] [ {id=col-c1 material=rc role=column detailing-compatible-with-q=true min-dimension-m=0.35m rho=0.012 rho-prime=0 omega-wd=0.12 steel-section-class=0} {id=beam-b1 material=rc role=beam detailing-compatible-with-q=true min-dimension-m=0.3m rho=0.01 rho-prime=0.005 omega-wd=0.08 steel-section-class=0} ] true true ct 0 0.075 ductile 0.5 false false 0 0.05
}
bridges [id:TEXT period-ratio:NUM fundamental-period-s:QTY v-rd-n:QTY bearing-d-rd-m:QTY permanent-gk-n:QTY correlated-occupancy:BOOL variables:TABLE] {
  br-1 2 1.2s 2000000N 0.4m 8000000N true [ {id=br-q1 category=F qk-n=1500000N} ]
}
assessments [id:TEXT knowledge-level:TEXT limit-state:TEXT supported-building-id:TEXT r-k-n:QTY gamma-el:NUM] {
  as-1 kl2 sd bldg-office 500000N 1
}
silos [id:TEXT height-m:QTY radius-m:QTY permanent-gk-n:QTY content-qk-n:QTY content-category:TEXT filling-ratio:NUM n-rd-n:QTY v-rd-n:QTY q-nominal:NUM] {
  si-1 12m 4m 1200000N 4000000N E 0.85 2000000N 1500000N 1.2
}
tanks [id:TEXT height-m:QTY radius-m:QTY permanent-gk-n:QTY content-qk-n:QTY content-category:TEXT filling-ratio:NUM v-rd-n:QTY] {
  tk-1 10m 5m 800000N 5000000N E 0.9 3000000N
}
foundations [id:TEXT supported-building-id:TEXT area-m2:QTY p-rd-pa:QTY h-rd-n:QTY k-foundation:NUM k-soil:NUM] {
  fd-1 bldg-office 200m2 400000Pa 500000N 800000 300000
}
retaining-walls [id:TEXT height-m:QTY phi-deg:NUM soil-gamma:NUM r:NUM h-rd-n-per-m:NUM] {
  rw-1 4.5m 32 18000 1.5 250000
}
towers [id:TEXT height-m:QTY m-rd-nm:NUM is-chimney:BOOL q-nominal:NUM permanent-gk-n:QTY correlated-occupancy:BOOL variables:TABLE] {
  tw-1 45m 8000000 false 2.5 1000000N true [ {id=tw-q1 category=E qk-n=50000N} ]
}
