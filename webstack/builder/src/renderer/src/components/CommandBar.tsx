import { KBarAnimator, KBarPortal, KBarPositioner, KBarSearch } from 'kbar'
import RenderResults from './RenderResults'

function CommandBar(): JSX.Element {
    return (
        <KBarPortal>
            <KBarPositioner className="kbar-positioner">
                <KBarAnimator className="animator">
                    <KBarSearch className="search" />
                    <RenderResults className="results" />
                </KBarAnimator>
            </KBarPositioner>
        </KBarPortal>
    )
}

export default CommandBar
