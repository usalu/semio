from lib import *
A="kitchen-extractor-hood"
def build(sc,asset):
    F=[];P=[]
    if sc==50:
        F.append(pl("canopy","canopy",[(-0.45,0.02),(0.45,0.02),(0.45,0.2),(-0.45,0.2)],z=3,tone=0.55,region="canopy",material="stainless-steel"))
        F.append(line("panel-edge","canopy",[(-0.435,0.06),(0.435,0.06)],z=3.3,tone=0.35))
        F.append(rc("filter","canopy",-0.43,0,0.43,0.02,z=3.1,tone=0.7,region="filter",material="filter-mesh"))
        for id_,x in (("lamp-a",-0.2),("lamp-b",0.2)): F.append(rc(id_,"canopy",x-0.03,0,x+0.03,0.012,z=3.2,tone=0.95,region="lamps",material="lamp-diffuser"))
        F.append(rc("chimney-lower","chimney",-0.15,0.2,0.15,0.55,z=3.4,tone=0.6,region="chimney",material="stainless-steel"))
        F.append(rc("chimney-upper","chimney-top",-0.14,0.55,0.14,0.9,z=3.5,tone=0.65,region="chimney",material="stainless-steel"))
        F.append(rc("collar","chimney-top",-0.1,0.9,0.1,0.915,z=3.6,tone=0.45,region="chimney",material="stainless-steel"))
        F.append(line("seam","chimney-top",[(-0.14,0.55),(0.14,0.55)],z=3.55,tone=0.3))
        P.append(rc("canopy","canopy",0,-0.45,0.5,0.45,z=3,tone=0.55,region="canopy",material="stainless-steel"))
        P.append(line("panel-edge","canopy",[(0.4,-0.45),(0.4,0.45)],z=3.3,tone=0.35))
        P.append(line("panel-lip","canopy",[(0.485,-0.435),(0.485,0.435)],z=3.3,tone=0.35))
        P.append(rc("chimney-lower","chimney",0,-0.15,0.3,0.15,z=3.4,tone=0.6,region="chimney",material="stainless-steel"))
        P.append(rc("chimney-upper","chimney-top",0,-0.14,0.28,0.14,z=3.5,tone=0.65,region="chimney",material="stainless-steel"))
        P.append(rc("collar","chimney-top",0.04,-0.1,0.24,0.1,z=3.6,tone=0.45,region="chimney",material="stainless-steel"))
        dec={"silhouette":("Front view: the 0.9 m canopy with its trim line, the hanging filter strip with the two lamps, and the 0.3 m chimney stepping to 0.28 m with the collar. Top view: the 0.5 m by 0.9 m canopy with the two trim lines of the sloping front panel and the chimney footprint with its collar.",
                           "Vorderansicht: die 0,9 m breite Haube mit ihrer Zierlinie, der hängende Filterstreifen mit den beiden Lampen und der 0,3 m breite Kamin, der auf 0,28 m abgesetzt ist, mit dem Kragen. Draufsicht: die Haube von 0,5 m mal 0,9 m mit den beiden Zierlinien der geneigten Frontblende und der Kaminfläche mit ihrem Kragen."),
             "omitted":("Front and top view leave the lamp pairs behind the front ones and the inside of the canopy out; the filter and the lamps are hidden from above.",
                        "Vorder- und Draufsicht lassen die hinteren Lampen und das Innere der Haube weg; Filter und Lampen sind von oben verdeckt.")}
    elif sc==100:
        F.append(pl("canopy","canopy",[(-0.45,0.025),(0.45,0.025),(0.45,0.2),(-0.45,0.2)],z=3,tone=0.55,region="canopy",material="stainless-steel"))
        F.append(rc("filter","canopy",-0.43,0,0.43,0.025,z=3.1,tone=0.75,regions=["filter","lamps"],material="filter-mesh"))
        F.append(rc("chimney-lower","chimney",-0.15,0.2,0.15,0.55,z=3.4,tone=0.6,region="chimney",material="stainless-steel"))
        F.append(rc("chimney-upper","chimney-top",-0.135,0.55,0.135,0.9,z=3.5,tone=0.65,region="chimney",material="stainless-steel"))
        F.append(rc("collar","chimney-top",-0.105,0.9,0.105,0.92,z=3.6,tone=0.45,region="chimney",material="stainless-steel"))
        P.append(rc("canopy","canopy",0,-0.45,0.5,0.45,z=3,tone=0.55,region="canopy",material="stainless-steel"))
        P.append(rc("chimney-lower","chimney",0,-0.15,0.3,0.15,z=3.4,tone=0.6,region="chimney",material="stainless-steel"))
        P.append(rc("chimney-upper","chimney-top",0,-0.135,0.27,0.135,z=3.5,tone=0.65,region="chimney",material="stainless-steel"))
        P.append(rc("collar","chimney-top",0.03,-0.105,0.24,0.105,z=3.6,tone=0.45,region="chimney",material="stainless-steel"))
        dec={"silhouette":("Front view: the 0.9 m canopy, the filter strip with the lamps merged, and the chimney in two steps with its collar. Top view: the 0.5 m by 0.9 m canopy with the chimney footprint in three steps.",
                           "Vorderansicht: die 0,9 m breite Haube, der Filterstreifen mit den zusammengefassten Lampen und der zweistufige Kamin mit seinem Kragen. Draufsicht: die Haube von 0,5 m mal 0,9 m mit der dreistufigen Kaminfläche."),
             "omitted":("Front and top view omit the trim lines and the individual lamps; from above the filter is hidden.",
                        "Vorder- und Draufsicht lassen die Zierlinien und die einzelnen Lampen weg; von oben ist der Filter verdeckt.")}
    else:
        F.append(pl("canopy","canopy",[(-0.45,0),(0.45,0),(0.45,0.2),(-0.45,0.2)],z=3,tone=0.55,regions=["canopy","filter","lamps"],material="stainless-steel"))
        F.append(rc("chimney-lower","chimney",-0.15,0.2,0.15,0.55,z=3.4,tone=0.6,region="chimney",material="stainless-steel"))
        F.append(rc("chimney-upper","chimney-top",-0.15,0.55,0.15,0.92,z=3.5,tone=0.65,region="chimney",material="stainless-steel"))
        P.append(rc("canopy","canopy",0,-0.45,0.5,0.45,z=3,tone=0.55,region="canopy",material="stainless-steel"))
        P.append(rc("chimney","chimney",0,-0.15,0.3,0.15,z=3.4,tone=0.6,region="chimney",material="stainless-steel"))
        dec={"silhouette":("Front view: a 0.9 m by 0.2 m canopy block under a 0.3 m chimney in two pieces. Top view: the 0.5 m by 0.9 m canopy with the 0.3 m chimney footprint.",
                           "Vorderansicht: ein Haubenblock von 0,9 m mal 0,2 m unter einem 0,3 m breiten Kamin aus zwei Teilen. Draufsicht: die Haube von 0,5 m mal 0,9 m mit der 0,3 m großen Kaminfläche."),
             "omitted":("Front and top view omit the collar, the trim lines and the lamps.",
                        "Vorder- und Draufsicht lassen Kragen, Zierlinien und Lampen weg.")}
    return F,P,dec
if __name__=="__main__": apply(A,build)
