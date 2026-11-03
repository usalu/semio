@capability-en1992-1-evaluate
@oracle-en1992-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1992 verification catalogue worked examples
  Every catalogue entry returns the hand-checked value of the specification example for the given national annex.

  @id-catalogue-example
  @level-quick
  @mode-differential
  Scenario Outline: Catalogue entry <entry> output <key>
    Given the catalogue entry "<entry>" for annex "<annex>" with inputs "<inputs>"
    When the entry is evaluated
    Then the output "<key>" equals <expected> within relative tolerance <tolerance>
    Examples:
      | id                 | entry | annex | inputs                                                  | key        | expected     | tolerance |
      | s-03-as1-de        | S-03  | De    | fck=30;b=300;d=450;m_ed=250000000                       | as1        | 0.001487     | 0.002     |
      | v-02-v-rd-c-de     | V-02  | De    | fck=30;bw=300;d=450;asl=1257                            | v_rd_c     | 68270        | 0.005     |
      | v-02-v-rd-c-en     | V-02  | En    | fck=30;bw=300;d=450;asl=1257                            | v_rd_c     | 81900        | 0.005     |
      | v-03-cot-theta-de  | V-03  | De    | fck=30;bw=300;z=405;v_ed=200000                         | cot_theta  | 2.194        | 0.002     |
      | v-03-cot-theta-en  | V-03  | En    | fck=30;bw=300;z=405;v_ed=200000                         | cot_theta  | 2.5          | 0.000001  |
      | a-01-phi-inf-de    | A-01  | De    | fck=30;rh=50;h0=187.5;t0=28                             | phi_inf    | 2.389        | 0.002     |
      | a-02-eps-cs-inf-de | A-02  | De    | fck=30;rh=50;h0=187.5;t0=28                             | eps_cs_inf | 0.0004689    | 0.002     |
      | m-11-eps-cs-inf-de | M-11  | De    | fck=30;rh=50;h0=187.5;t0=28                             | eps_cs_inf | 0.0004689    | 0.002     |
      | b-02-lb-rqd-de     | B-02  | De    | fck=30;phi=20                                           | lb_rqd     | 0.8525       | 0.002     |
      | b-03-lbd-de        | B-03  | De    | fck=30;phi=20;cd=45                                     | lbd        | 0.6927       | 0.002     |
      | p-03-v-ed-de       | P-03  | De    | fck=30;d=210;c1=400;v_ed=700000;rho_ly=0.01;rho_lz=0.01 | v_ed       | 865000       | 0.002     |
      | pa-01-f-rdu-de     | PA-01 | De    | fck=30;ac0=40000;ac1=360000                             | f_rdu      | 2040000      | 0.000001  |
      | fa-03-n-rk-c-de    | FA-03 | De    | fck=30;hef=100                                          | n_rk_c     | 48750        | 0.001     |
      | sl-07-ld-limit-de  | SL-07 | De    | fck=30;b=300;d=450;as=445.5;span=9000                   | ld_limit   | 34.03        | 0.002     |
      | a-15-util-h-de     | A-15  | De    | l0t=6000;b=200;h=400                                    | util_h     | 0.8          | 0.000001  |
      | af-01-as-x-de      | AF-01 | De    | sx=2;sy=1;txy=3;t=200;fck=30                            | as_x       | 2.3          | 0.000001  |
      | af-01-as-y-de      | AF-01 | De    | sx=-4;sy=1;txy=2;t=200;fck=30                           | as_y       | 0.92         | 0.000001  |
      | aj-03-f-td-de      | AJ-03 | De    | f_ed=300000;a_c=150;z0=300;fck=30                       | f_td       | 150000       | 0.000001  |
      | t-05-as-req-de     | T-05  | De    | bimoment=4000000000;h_s=400;t_f=150;d_f=250;fck=30      | as_req     | 0.0000950830 | 0.0001    |
      | ak-01-xi-de        | AK-01 | De    | fck=30;phi=2.5                                          | xi         | 0.917915     | 0.00001   |
      | al-01-as-x-bot-de  | AL-01 | De    | nx=100;mx=20000;h=300;a=40;fck=30                       | as_x_bot   | 0.32409      | 0.0001    |
      | b-07-lpt-t3-en    | B-07  | En    | fck=30;phi=12.5;sigma_pm0=1400;t=3;tendon:strand        | lpt        | 1.302648     | 0.00001   |
      | t-02-asw-s-de      | T-02  | De    | fck=30;b=300;h=500;t_ed=20000000;cot_theta=1            | asw_s      | 0.2745       | 0.002     |
      | t-02-asl-de        | T-02  | De    | fck=30;b=300;h=500;t_ed=20000000;cot_theta=1            | asl        | 0.0003363    | 0.003     |
      | t-01-tef-link-de | T-01  | De    | b=300;h=550;c=45;phi_l=25                              | tef        | 0.115        | 0.00001 |
      | t-02-links-util-de | T-02  | De    | fck=30;b=400;h=600;t_ed=35000000;cot_theta=1.5;c=30;phi_l=20;asw_s_provided=0.9;asl_provided=2200 | util_links | 0.2218364 | 0.00001 |
      | t-02-long-bending-de | T-02  | De    | fck=30;b=400;h=600;t_ed=35000000;cot_theta=1.5;c=30;phi_l=20;asl_provided=1000;asl_bending=700;tension_share=0.5 | util_long | 1.0414063 | 0.00001 |
      | st-01-cracked-de | ST-01 | De    | fck=30;strut_force=300000;b=300;w=100;kind=cracked | util | 0.8912656 | 0.00001 |
      | st-03-cct-de | ST-03 | De    | fck=30;sigma_c=12;node=CCT | util | 1.0695187 | 0.00001 |
      | t-04-util-max-de   | T-04  | De    | fck=30;b=300;h=500;t_ed=20000000;cot_theta=1;bw=300;d=450;asl=1257;v_ed=150000 | util_max | 0.3933 | 0.002 |
      | t-04-t-rd-max-de   | T-04  | De    | fck=30;b=300;h=500;t_ed=20000000;cot_theta=1;bw=300;d=450;asl=1257;v_ed=150000 | t_rd_max | 100154.11 | 0.0001 |
      | a-10-e2-de         | A-10  | De    | fck=30;b=400;h=400;n_ed=1200000;l0=6000;as=1600;is=160;phi_ef=1.2 | e2 | 0.05517953 | 0.00001 |
      | dt-09-scl-max-de  | DT-09 | De    | ac=90000;phi=20                                         | scl_max    | 0.24         | 0.000001  |
      | pr-05-relax-c2-de  | PR-05 | De    | ep=195000;ecm=33000;phi=2;ap=1000;ac=150000;ic=3.1e9;zcp=100;eps_cs=0.0003;sigma_c_qp=8;sigma_pi=1300;fpk=1860;t_hours=500000;relax_class=2;rho1000=2.5 | relax      | 50475245.25  | 0.0001    |
      | pr-05-loss-c2-de   | PR-05 | De    | ep=195000;ecm=33000;phi=2;ap=1000;ac=150000;ic=3.1e9;zcp=100;eps_cs=0.0003;sigma_c_qp=8;sigma_pi=1300;fpk=1860;t_hours=500000;relax_class=2;rho1000=2.5 | loss       | 167906490.32 | 0.0001    |
      | pr-05-relax-c1-de  | PR-05 | De    | ep=195000;ecm=33000;phi=2;ap=1000;ac=150000;ic=3.1e9;zcp=100;eps_cs=0.0003;sigma_c_qp=8;sigma_pi=1300;fpk=1860;t_hours=500000;relax_class=1;rho1000=8   | relax      | 246478632.68 | 0.0001    |
      | pr-05-loss-c1-de   | PR-05 | De    | ep=195000;ecm=33000;phi=2;ap=1000;ac=150000;ic=3.1e9;zcp=100;eps_cs=0.0003;sigma_c_qp=8;sigma_pi=1300;fpk=1860;t_hours=500000;relax_class=1;rho1000=8   | loss       | 304021801.77 | 0.0001    |
      | m-02-fcm-de | M-02 | De | fck=30 | fcm | 38000000 | 0.000001 |
      | m-02-fctm-de | M-02 | De | fck=30 | fctm | 2900000 | 0.000001 |
      | m-02-ecm-de | M-02 | De | fck=30 | ecm | 33000000000 | 0.000001 |
      | m-08-sigma-inclined-de | M-08 | De | fyk=500;es=200000;k=1.08;eps_uk=50;eps=3 | sigma_s | 435453542.2644 | 0.00001 |
      | m-08-eps-ud-de | M-08 | De | fyk=500;es=200000;k=1.08;eps_uk=50;eps=3 | eps_ud | 45 | 0.000001 |
      | b-01-fbd-good-de | B-01 | De | fck=30;phi=20;bond=good | fbd | 2550000 | 0.000001 |
      | b-01-fbd-poor-de | B-01 | De | fck=30;phi=20;bond=poor | fbd | 1785000 | 0.000001 |
      | a-16-sigma-limit-de | A-16 | De | ap=1000;fpk=1860;fp01k=1640;pm0=1200000 | sigma_pm0_max | 1394000000 | 0.000001 |
      | a-16-util-de | A-16 | De | ap=1000;fpk=1860;fp01k=1640;pm0=1200000 | util | 0.8608321 | 0.00001 |
      | dt-14-peripheral-de | DT-14 | De | li=6;tie=peripheral;as_provided=150 | f_tie | 60000 | 0.000001 |
      | dt-14-peripheral-util-de | DT-14 | De | li=6;tie=peripheral;as_provided=150 | util | 0.8 | 0.000001 |
      | dt-14-internal-de | DT-14 | De | li=6;width_m=8;tie=internal | f_tie | 160000 | 0.000001 |
      | dt-14-column-de | DT-14 | De | li=6;wall_length_m=5;tie=column | f_tie | 750000 | 0.000001 |
