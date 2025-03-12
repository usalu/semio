import * as React from 'react'
import Diagram from '@semio/core/components/ui/Diagram'

const IndexPage = () => {
    return (
        <main>
            <h1 className="bg-amber-600 text-3xl font-bold underline">
                Hello world!
            </h1>
            <Diagram />
        </main>
    )
}

export const Head = () => <title>Home Page</title>

export default IndexPage