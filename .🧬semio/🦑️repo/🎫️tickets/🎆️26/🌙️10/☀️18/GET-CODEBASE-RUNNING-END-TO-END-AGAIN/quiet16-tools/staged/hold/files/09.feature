Feature: An Action Takes One Job Of The Grant And Waits Without A Handshake Clock

  Scenario: Births That Fill The Grant Never Block Their Own Children
    Given a build whose running action births hold every job of the grant (1, 3 or 8)
    When each action's child joins the control for its own execution
    Then every execution join starts at once, because it belongs to its birth's job

  Scenario: A Growth Longer Than The Handshake Does Not Fail The Action
    Given a quiet grant of one job whose growth takes longer than the birth handshake
    When two actions are born
    Then each hears that its birth is waiting, its handshake clock stops, and both finish one after the other
