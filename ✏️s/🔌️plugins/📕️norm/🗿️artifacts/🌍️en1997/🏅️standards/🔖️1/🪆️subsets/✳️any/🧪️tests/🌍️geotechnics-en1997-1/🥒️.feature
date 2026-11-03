@capability-en1997-1-geotechnics-design
@oracle-en1997-1-geotechnics-groundhog-scipy
@comparison-method-spread-v1
Feature: EN 1997 design-case geotechnics against groundhog and scipy with registry partial factors
  Every case of the committed geotechnics case file names an annex, a design approach and a design situation. The partial factors are not inputs: both roles read them from the NDP registry rows of that set (`ndp.en1997.{a1,a2,m1,m2,r1,r2,r3,de.*}`, all still unverified), the Rust subject through the catalogue entries GEO-F01, RW-06, SLS-02 and SLOPE-01, the Python oracle from the registry file with groundhog (bearing, Rankine, consolidation), scipy quadrature (Boussinesq) and a 400-slice root-finding simplified Bishop. The factors, the design resistances, the utilisations and the factor of safety must agree.

  @id-oracle-comparison
  @level-quick
  @mode-differential
  Scenario Outline: Compare the quantities of <fixture>
    Given the committed oracle input shared://🌍️geotechnics/<fixture>
    When the Python oracle and the kernel both evaluate it
    Then every quantity named by the oracle agrees within the comparison profile
    Examples:
      | id | fixture |
      | geotechnics-design-cases | 🔣️.json |
