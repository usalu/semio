"""Shared literature, materials and thermal reference data of the CV-FO assets (ISO 10456 design values, Incropera, Kays/Schubert)."""
from lib import T, layer, region

REF_ISO10456 = {'id': 'iso-10456-2007', 'citation': 'ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values (conductivity, specific heat capacity and density of timber, steel, aluminium, concrete, plastics).'}
REF_INCROPERA = {'id': 'incropera-2018', 'citation': 'Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (surface radiative properties, convection correlations).'}
REF_WOOD = {'id': 'wood-handbook-2021', 'citation': 'Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service (density and elastic modulus of oak and beech; rounded air-dry estimates).'}
REF_EN1993 = {'id': 'en-1993-1-1', 'citation': 'EN 1993-1-1:2005 Eurocode 3: Design of steel structures, Part 1-1: General rules (E = 210 GPa, density 7850 kg/m3).'}
REF_EN1991 = {'id': 'en-1991-1-1', 'citation': 'EN 1991-1-1:2002 Eurocode 1: Actions on structures, Part 1-1: General actions, densities, self-weight, imposed loads for buildings.'}
REF_ASHBY = {'id': 'ashby-2011', 'citation': 'Ashby, M. F. (2011). Materials Selection in Mechanical Design, 4th ed., Butterworth-Heinemann (densities and properties of polymers, elastomers and metals).'}
REF_EN1991_1_4 = {'id': 'en-1991-1-4', 'citation': 'EN 1991-1-4:2005 Eurocode 1: Actions on structures, Part 1-4: Wind actions.'}
REF_EN1999 = {'id': 'en-1999-1-1', 'citation': 'EN 1999-1-1:2007 Eurocode 9: Design of aluminium structures, Part 1-1: General structural rules (E = 70 GPa, density 2700 kg/m3).'}
REF_EN1992 = {'id': 'en-1992-1-1', 'citation': 'EN 1992-1-1:2004 Eurocode 2: Design of concrete structures, Part 1-1 (density of normal-weight concrete 2400 kg/m3).'}
REF_SCHUBERT = {'id': 'schubert-2006', 'citation': 'Schubert, E. F. (2006). Light-Emitting Diodes, 2nd ed., Cambridge University Press, chapter on efficiency and thermal management (the electrical power of a white LED not emitted as visible radiation, typically more than half, is dissipated as heat in the package and luminaire housing).'}
REF_HALLIDAY = {'id': 'pendulum-halliday', 'citation': 'Halliday, D., Resnick, R., Walker, J. (2014). Fundamentals of Physics, 10th ed., Wiley, chapter 15: small-angle pendulum period T = 2 pi sqrt(L/g).'}
REF_ISO7730 = {'id': 'iso-7730-2005', 'citation': 'ISO 7730:2005 Ergonomics of the thermal environment - Analytical determination and interpretation of thermal comfort (typical outer clothing surface temperatures of seated persons around 28-32 degC in comfort conditions).'}

PENETRATION_S = 3600


def effective_cap(rho, cp, k, t, tau=PENETRATION_S):
    """Areal heat capacity J/(m2 K) taking part over the time `tau`: rho*cp*min(t, sqrt(alpha*tau))."""
    delta = (k / (rho * cp) * tau) ** 0.5
    return round(rho * cp * min(t, delta))


MAT = {
    'oak': dict(k=0.18, cp=1600, rho=700),
    'beech': dict(k=0.18, cp=1600, rho=720),
    'steel': dict(k=50.0, cp=450, rho=7800),
    'aluminium': dict(k=160.0, cp=880, rho=2800),
    'concrete': dict(k=1.65, cp=1000, rho=2300),
    'hdpe': dict(k=0.50, cp=1800, rho=980),
    'pa': dict(k=0.25, cp=1600, rho=1150),
    'rubber': dict(k=0.25, cp=1000, rho=1150),
    'particleboard': dict(k=0.13, cp=1700, rho=650),
    'polycarbonate': dict(k=0.20, cp=1200, rho=1200),
}
