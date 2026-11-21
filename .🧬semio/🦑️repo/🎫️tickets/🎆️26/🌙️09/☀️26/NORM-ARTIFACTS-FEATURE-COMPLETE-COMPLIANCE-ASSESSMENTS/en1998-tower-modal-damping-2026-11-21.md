# EN 1998 tower, mast and chimney modal damping

C98-09 evaluates the equivalent damping of one mode. It uses component strain energies from the structural model and the associated structural material, lining and foundation damping ratios. The damage-limitation and design earthquake cases use the respective lower and upper lining ranges. Foundation damping is an explicit choice within the representative intervals. The calculated result is checked against the 15% recommendation. The separate experimental input accepts a substantiated modal damping value. [EN 1998-6:2005 §4.2.4 and Annex B](https://www.phd.eng.br/wp-content/uploads/2014/11/en.1998.6.2005.pdf) · [EN 1998-2:2005 §4.1.3](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.2.2005.pdf)

The case applies the damping correction inside the applicable branch of the horizontal elastic spectrum, then multiplies the acceleration by the supplied effective modal mass. C98-02 uses the same corrected implementation. This keeps the spectral acceleration at zero period equal to ground acceleration times the site factor. The correction is applied to the elastic spectrum. The existing automatic tower design-spectrum analysis uses a fixed modal damping ratio for CQC. [EN 1998-6:2005 §4.2.4](https://www.phd.eng.br/wp-content/uploads/2014/11/en.1998.6.2005.pdf)

The production Rust functions and independent SQLite queries matched 11 published component entries and 174 spectrum cases. The scenarios included all spectrum branches, experimental damping, mixed component energy, a negative energy, zero total energy and foundation damping outside the published interval. Native catalogue and browser builds remain pending.

## Guyed steel masts

C98-10 checks the strict relationship between seismic stress variation and cable preload and the low-dissipative behaviour factor. It evaluates the horizontal stiffness of one cable with a supplied cable modulus or an individual strand modulus with the wrapping correction. The sag calculation uses cable and design ice weight at a specified tensile equilibrium. The project confirms that the separate mast analysis accounts for cable mass and significant impulse effects. [EN 1998-6:2005 §§4.2.2–4.2.3 and 8.2](https://www.phd.eng.br/wp-content/uploads/2014/11/en.1998.6.2005.pdf)

Rust and SQLite matched 116 cable stiffness calculations, including nonzero sag and excluded geometry. The native catalogue run is queued.
