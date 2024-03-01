import {
    // ...
    KBarResults,
    useMatches
} from 'kbar'
import ResultItem from './ResultItem'

// interface RenderResultsProps {
//     className?: string;
// }

// function RenderResults({ className }: RenderResultsProps) : JSX.Element {
//     const { results } = useMatches();

//     return (
//         <KBarResults
//             items={results}
//             onRender={({ item, active }) =>
//                 typeof item === "string" ? (
//                     <div >{item}</div>
//                 ) : (
//                     <div
//                         // style={{
//                         //     background: active ? "#eee" : "transparent",
//                         //   }}
//                         className={active? `${className} active` : className}
//                     >
//                         {item.name}
//                     </div>
//                 )
//             }
//         />
//     );
// }

interface RenderResultsProps {
    className?: string
}

function RenderResults({ className }: RenderResultsProps): JSX.Element {
    const { results, rootActionId } = useMatches()

    return (
        <KBarResults
            items={results}
            onRender={({ item, active }) =>
                typeof item === 'string' ? (
                    <div className={active ? `${className} active` : className}>{item}</div>
                ) : (
                    <ResultItem action={item} active={active} currentRootActionId={rootActionId} />
                )
            }
        />
    )
}

export default RenderResults
