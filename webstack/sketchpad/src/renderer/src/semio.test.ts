import { convertCoordinateSystemToTransform, convertTransformToCoordinateSystem } from './semio'
import { CoordinateSystem } from './semio'
import { Matrix4 } from 'three'
import { expect, test } from 'vitest'

expect.extend({
    toBeCloseToMatrix(received: Matrix4, expected: Matrix4) {
        const pass = received.elements.every(
            (val, i) => Math.abs(val - expected.elements[i]) < Number.EPSILON
        )
        if (pass) {
            return {
                message: () =>
                    `expected ${received.elements} not to be close to ${expected.elements}`,
                pass: true
            }
        } else {
            return {
                message: () => `expected ${received.elements} to be close to ${expected.elements}`,
                pass: false
            }
        }
    },
    toBeCloseToCoordinateSystem(received: CoordinateSystem, expected: CoordinateSystem) {
        const pass = Object.keys(received).every((key) => {
            const val = received[key as keyof CoordinateSystem]
            const exp = expected[key as keyof CoordinateSystem]
            if (typeof val === 'number') {
                return Math.abs(val - exp) < Number.EPSILON
            } else {
                return Object.keys(val).every(
                    (k) =>
                        Math.abs(val[k as keyof typeof val] - exp[k as keyof typeof exp]) <
                        Number.EPSILON
                )
            }
        })
        if (pass) {
            return {
                message: () => `expected ${received} not to be close to ${expected}`,
                pass: true
            }
        } else {
            return {
                message: () => `expected ${received} to be close to ${expected}`,
                pass: false
            }
        }
    }
})

test('convertCoordinateSystemToTransform origin', () => {
    expect(
        convertCoordinateSystemToTransform({
            origin: { x: 1, y: 2, z: 3 },
            xAxis: { x: 1, y: 0, z: 0 },
            yAxis: { x: 0, y: 1, z: 0 }
        } as CoordinateSystem)
    ).toBeCloseToMatrix(new Matrix4().set(
        1, 0, 0, 1,
        0, 0, 1, 3,
        0, -1, 0, -2,
        0, 0, 0, 1))
})

test('convertTransformToCoordinateSystem origin', () => {
    expect(convertTransformToCoordinateSystem(new Matrix4().set(
        1, 0, 0, 1,
        0, 1, 0, 3,
        0, 0, 1, -2,
        0, 0, 0, 1)
    )).toBeCloseToCoordinateSystem({
        origin: { x: 1, y: 2, z: 3 },
        xAxis: { x: 1, y: 0, z: 0 },
        yAxis: { x: 0, y: 0, z: 1 }
    } as CoordinateSystem)
}
)

test('convertCoordinateSystemToTransform zy', () => {
    expect(
        convertCoordinateSystemToTransform({
            origin: { x: 0, y: 0, z: 0 },
            xAxis: { x: 0, y: 0, z: 1 },
            yAxis: { x: 0, y: 1, z: 0 }
        } as CoordinateSystem)
    ).toBeCloseToMatrix(new Matrix4().set(
        0, 0, -1, 0,
        1, 0, 0, 0,
        0, -1, 0, 0,
        0, 0, 0, 1))
})

test('convertTransformToCoordinateSystem zy', () => {
    const coordinateSystem = convertTransformToCoordinateSystem(new Matrix4().set(
        0, -1, 0, 0,
        1, 0, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1))
    expect(
        coordinateSystem
    ).toBeCloseToCoordinateSystem({
        origin: { x: 0, y: 0, z: 0 },
        xAxis: { x: 0, y: 0, z: 1 },
        yAxis: { x: -1, y: 0, z: 0 }
    } as CoordinateSystem)
})
