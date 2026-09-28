semio norm.en1998.dsl v1
annex=de site=seismic-zone=zone2 a-gr=0.6m/s2 de-ground-combo=B-R en-ground-type="" en-spectrum-type="" importance-class=II
buildings [id:TEXT name:TEXT plan-width-m:QTY plan-length-m:QTY systems:TABLE storeys:TABLE members:TABLE plan-regular:BOOL elevation-regular:BOOL t1-method:TEXT t1-given-s:NUM ct:NUM drift-limit-class:TEXT nu:NUM multiple-resisting-systems:BOOL claims-simple-masonry:BOOL masonry-wall-area-ratio:NUM accidental-eccentricity-ratio:NUM] {
  bldg-weak "Weak RC frame" 24m 16m [ {id=sys-x direction=x system-type=frame material=rc ductility-class=dcm q0=3 alpha-u-over-alpha-1=1.3 k-w=1 base-shear-resistance-n=80000N} {id=sys-y direction=y system-type=frame material=rc ductility-class=dcm q0=3 alpha-u-over-alpha-1=1.3 k-w=1 base-shear-resistance-n=80000N} ] [ {id=s1 height-m=3.5m permanent-gk-n=2334780N correlated-occupancy=true stiffness-x=45000000 stiffness-y=45000000 centre-of-mass-x-m=12m centre-of-mass-y-m=8m centre-of-stiffness-x-m=12m centre-of-stiffness-y-m=8m drift-x-m=0.045m drift-y-m=0.045m shear-resistance-n=50000N
  variables [id:TEXT category:TEXT qk-n:QTY] {
    s1-q B 1716750N
  }
  } {id=s2 height-m=3.5m permanent-gk-n=2251395N correlated-occupancy=true stiffness-x=42500000 stiffness-y=42500000 centre-of-mass-x-m=12m centre-of-mass-y-m=8m centre-of-stiffness-x-m=12m centre-of-stiffness-y-m=8m drift-x-m=0.045m drift-y-m=0.045m shear-resistance-n=50000N
  variables [id:TEXT category:TEXT qk-n:QTY] {
    s2-q B 1655437.5N
  }
  } {id=s3 height-m=3.5m permanent-gk-n=2168010N correlated-occupancy=true stiffness-x=40000000 stiffness-y=40000000 centre-of-mass-x-m=12m centre-of-mass-y-m=8m centre-of-stiffness-x-m=12m centre-of-stiffness-y-m=8m drift-x-m=0.045m drift-y-m=0.045m shear-resistance-n=50000N
  variables [id:TEXT category:TEXT qk-n:QTY] {
    s3-q B 1594125N
  }
  } {id=s4 height-m=3.5m permanent-gk-n=2001240N correlated-occupancy=true stiffness-x=37500000 stiffness-y=37500000 centre-of-mass-x-m=12m centre-of-mass-y-m=8m centre-of-stiffness-x-m=12m centre-of-stiffness-y-m=8m drift-x-m=0.045m drift-y-m=0.045m shear-resistance-n=50000N
  variables [id:TEXT category:TEXT qk-n:QTY] {
    s4-q B 1177200N
  }
  } ] [ {id=col-c1 material=rc role=column detailing-compatible-with-q=false min-dimension-m=0.2m rho=0.002 rho-prime=0 omega-wd=0.02 steel-section-class=0} {id=beam-b1 material=rc role=beam detailing-compatible-with-q=false min-dimension-m=0.2m rho=0.002 rho-prime=0 omega-wd=0.02 steel-section-class=0} ] false false ct 0 0.075 ductile 0.5 false false 0 0.05
}
bridges [id:TEXT period-ratio:NUM fundamental-period-s:QTY v-rd-n:QTY bearing-d-rd-m:QTY permanent-gk-n:QTY correlated-occupancy:BOOL variables:TABLE] {
  br-weak 1.2 1.5s 50000N 0.02m 8000000N true [ {id=br-q1 category=F qk-n=1500000N} ]
}
assessments [id:TEXT knowledge-level:TEXT limit-state:TEXT supported-building-id:TEXT r-k-n:QTY gamma-el:NUM] {
  as-weak kl1 nc bldg-weak 20000N 1.2
}
silos [id:TEXT height-m:QTY radius-m:QTY permanent-gk-n:QTY content-qk-n:QTY content-category:TEXT filling-ratio:NUM n-rd-n:QTY v-rd-n:QTY q-nominal:NUM] {
  si-weak 14m 3.5m 1200000N 5000000N E 1 50000N 40000N 1.5
}
tanks [id:TEXT height-m:QTY radius-m:QTY permanent-gk-n:QTY content-qk-n:QTY content-category:TEXT filling-ratio:NUM v-rd-n:QTY] {
  tk-weak 12m 4m 800000N 6000000N E 1 50000N
}
foundations [id:TEXT supported-building-id:TEXT area-m2:QTY p-rd-pa:QTY h-rd-n:QTY k-foundation:NUM k-soil:NUM] {
  fd-weak bldg-weak 40m2 80000Pa 20000N 200000 100000
}
retaining-walls [id:TEXT height-m:QTY phi-deg:NUM soil-gamma:NUM r:NUM h-rd-n-per-m:NUM] {
  rw-weak 6m 25 19000 1 20000
}
towers [id:TEXT height-m:QTY m-rd-nm:NUM is-chimney:BOOL q-nominal:NUM permanent-gk-n:QTY correlated-occupancy:BOOL variables:TABLE] {
  tw-weak 55m 100000 true 1.5 1500000N true [ ]
}
